use std::num::Float;
use std::collections::BTreeSet;

use voxel::Voxel;
use ray::Ray;
use vector::Vector;

mod voxel;
mod ray;
mod vector;

struct Voxelizer{
	lod:u8,
    limit:u64,
    r:u64,
    
    xlimit:u64,
    ylimit:u64,
    zlimit:u64,
    
    cx:i64,
    cy:i64,
    cz:i64,
    
    inside:u64,
    outside:u64,
    actual_total:u64,
    percentage:u64,
    inline:u64,
    counter:u64,
    
    voxel:Voxel,
    
    lod_list:Vec<u8>, //level of detail for each voxel
	lod_voxels:Vec<Voxel>, //voxels with the LOD
}

impl Voxelizer{

    fn new(lod:u8, limit:u64, r:u64)->Voxelizer{
        let mut voxel = Voxel::new();
        
        let xlimit = limit;
        let ylimit = limit;
        let zlimit = limit;
        
        let cx = (xlimit/2) as i64;
        let cy = (ylimit/2) as i64;
        let cz = (zlimit/2) as i64;
        
        Voxelizer{
        
            lod:lod, 
            limit:limit, 
            r:r, 
            
            xlimit:xlimit,
            ylimit:ylimit,
            zlimit:zlimit,

            cx:cx,
            cy:cy,
            cz:cz,
            
            inside:0,
            outside:0,
            actual_total:0,
            percentage:0,   
            inline:0,
            counter:0,
            
            voxel:voxel,
            
            lod_list:Vec::new(),
            lod_voxels:Vec::new()
            
          }
    }
    
    fn start(&mut self){
        for i in range (0, self.xlimit){
        let new_percentage = (i as f64/self.limit as f64 * 100.0) as u64;
        if new_percentage != self.percentage {
            println!("{} %", self.percentage);
        }
        self.percentage = new_percentage;
        for j in range (0, self.ylimit) {
            for k in range (0, self.zlimit){
                  self.actual_total += 1;
                  //sign matters here
                  let x = (i as i64 - self.cx) as f64;
                  let y = (j as i64 - self.cy) as f64;
                  let z = (k as i64 - self.cz) as f64;
                  let iijjkk:f64 = x*x + y*y + z*z ;
                  let sqrt_iijjkk:f64 = (iijjkk).sqrt();
                  let rounded_sqrt_ijk = (sqrt_iijjkk.round()) as u64;
                  
                  let index = i * self.ylimit * self.zlimit + j * self.zlimit + k;
                  let m = morton(i, j, k, self.lod);
                  assert!(index == self.counter);
                  
                  if rounded_sqrt_ijk == self.r {
                    self.inside += 1;
                    self.inline += 1;
                    self.voxel.set_bit(m, true);
                    println!("inside: {}, {}, {} morton: {}",i,j,k, m);
                  }
                  
                  else if sqrt_iijjkk < self.r as f64 {
                    self.inside += 1;
                    self.voxel.set_bit(m, true);
                    println!("inside: {}, {}, {} morton: {}",i,j,k, m);
                  }
                  else {
                    self.outside += 1;
                  }
                  self.counter += 1;
              }
           }    
        }
    }
    
    
    
    fn debug(&self){
        println!("lod: {}", self.lod);
        println!("limit: {}", self.limit);
        println!("radius: {}", self.r);
        println!("inside: {}", self.inside);
        println!("outside: {}", self.outside);
        println!("inline: {}", self.inline);
        println!("actual_total: {} or {}", self.actual_total, self.inside+self.outside);
    }
    
    
     //determines if the point is inside the boundary of this voxel
    fn bounded(&self, x:i64, y:i64, z:i64)->bool{
    	let xlowerbound = 0;
    	let ylowerbound = 0;
    	let zlowerbound = 0;
 		let xupperbound = self.xlimit as i64;
 		let yupperbound = self.ylimit as i64;
 		let zupperbound = self.zlimit as i64;
 		if x < xlowerbound || y < ylowerbound || z < zlowerbound 
 		|| x > xupperbound || y > yupperbound || z > zupperbound
 		{
 			return false;
 		}
 		true
    }
    
    //determine if it hits a voxel
    fn hit(&self, x:i64, y:i64, z:i64)->bool{
    	let m = morton(x as u64, y as u64, z as u64, self.lod);
    	let isset = self.voxel.isset(m);
    	isset
    }
    
    //get the x,y,z at given lod
    fn at_lod(&self, x:u64, y:u64, z:u64, new_lod:u8)->(u64, u64, u64){
    	//println!("current lod: {} --> {}", self.lod, new_lod);
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64).round() as u64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64).round() as u64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64).round() as u64;
    	//println!("new {},{},{}",xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    //build the voxel LOD's at each level of detail
    fn build_voxel_lod(&mut self){
    	 let mut parent_voxel = self.voxel.clone();
    	 for i in range(1, self.lod){
    	 	self.lod_list.push(i+1);
    	 	println!("Building voxel at LOD: {}", i);
    		parent_voxel = parent_voxel.parent();
    		self.lod_voxels.push(parent_voxel.clone());
    		parent_voxel.display_bitset();
    	}
    }
   
    
 }  

 
fn morton(x:u64, y:u64, z:u64, lod:u8)->u64{
	let mut answer:u64 = 0;
	for i in range(0, lod) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}

// decode a given 64-bit morton code to an integer (x,y,z) coordinate
fn morton_decode(morton:u64, lod:u8)->(u64, u64, u64){
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;
	for i in range (0, lod) {
		x |= ((morton & ( 1  << 3 * i + 0)) >> ((3 * i) + 0)-i);
		y |= ((morton & ( 1  << 3 * i + 1)) >> ((3 * i) + 1)-i);
		z |= ((morton & ( 1  << 3 * i + 2)) >> ((3 * i) + 2)-i);
	}
	(x, y, z)
}


 

fn main(){
	let lod:u8 = 5;
    let limit:u64 = 1 << lod;
    let r:u64 = 1 << lod-2;//do a radius of half the limit
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    voxelizer.start();
    //voxelizer.debug();
    //voxelizer.voxel.show_indexes();
    voxelizer.voxel.display_bitset();
    voxelizer.build_voxel_lod();
    
    
    //look at the center of the sphere
    let cx = (limit/2) as i64;
    let cy = (limit/2) as i64;
    let cz = (limit/2) as i64;

	//at -z
    let xorig = (limit as i64/2) + 10;
    let yorig = (limit as i64/2) + 10;
    let zorig = -(limit as i64);
    
    println!("origin: {}, {}, {}", xorig, yorig, zorig);
    println!("looking at: {}, {}, {}", cx, cy, cz);

	//compute unit vector
	
	let dx = (cx - xorig) as f64;
	let dy = (cy - yorig) as f64;
	let dz = (cz - zorig) as f64;
	
	let v = Vector{x:dx, y:dy, z:dz};
	
	println!("vector: {}",v);
	
    let r = Ray::new(xorig, yorig, zorig, v, lod);
    //ray march from 0 to 10
    let mut p = 0;

	

    loop {
    	let (px, py, pz) = r.at_length(p);
    	let bounded = voxelizer.bounded(px, py, pz);
    	if bounded {
			for detail in range (1, lod) {
				let (xlod,ylod,zlod) = voxelizer.at_lod(px as u64, py as u64, pz as u64, detail);
				println!("at lod: {} = {},{},{}",detail, xlod, ylod, zlod);
			}
    		let hit = voxelizer.hit(px, py, pz);
    		println!("p: {}, {}, {} --> {} ", px, py, pz, hit);
    	}
    	if px == cx && py == cy && pz == cy {
    		break;
    	}
    	p += 1;
    }
    
}


#[test]
fn morton_test() {
	let m = morton(1,2,3, 5);
	let d = morton_decode(m, 5);
	println!("morton: {}",m);
	assert!(d == (1,2,3));

	let m1 = morton(20,14,11, 5);
	let d1 = morton_decode(m1, 5);
	assert!(d1 == (20,14,11));

}
