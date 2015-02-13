use std::num::Float;
use std::collections::BTreeSet;

use voxel::Voxel;
use ray::Ray;
use vector::Vector;
use point::Point;
use screen::Screen;
use color::Color;
use morton::morton;

use std::old_io::File;

pub struct Voxelizer{
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
    
    voxel:Voxel, //the highest resolution voxel
    
    lod_list:Vec<u8>, //level of detail for each voxel
	pub lod_voxels:Vec<Voxel>, //voxels with the LOD
}

impl Voxelizer{

    pub fn new(lod:u8, limit:u64, r:u64)->Voxelizer{
        let mut voxel = Voxel::new(lod);
        
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
    
    pub fn start(&mut self){
        for i in range (0, self.xlimit){
        let new_percentage = ((i as f64/self.xlimit as f64) * 100.0).round() as u64;
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
                    self.voxel.set_bit_at_loc(i, j, k, true);
                    //println!("inside: {}, {}, {} morton: {}",i,j,k, m);
                  }
                  
                  else if sqrt_iijjkk < self.r as f64 {
                    self.inside += 1;
                    self.voxel.set_bit_at_loc(i, j, k, true);
                    //println!("inside: {}, {}, {} morton: {}",i,j,k, m);
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
    
    //determine if it hits a voxel, this is direct test wihout using the lower level LOD
    fn hit_direct(&self, x:i64, y:i64, z:i64)->bool{
    	let bounded = self.bounded(x, y, z);
    	if !bounded {
    		//println!("{} {} {} not bounded..",x, y, z);
    		return false;
    	}
    	let m = morton(x as u64, y as u64, z as u64, self.lod);
    	let isset = self.voxel.isset(m);
    	isset
    }
    
    //check if point at a certain LOD (level of detail) hits
     fn hit_at_lod(&self, x:i64, y:i64, z:i64, lod:u8)->bool{
     	let bounded = self.bounded(x, y, z);
     	if !bounded {
    		//println!("{} {} {} not bounded..",x, y, z);
    		return false;
    	}
    	let (xlod, ylod, zlod) = self.at_lod(x, y, z, lod);
    	let index = self.lod - (lod+1);
    	//println!("index: {}", index);
    	let m = morton(xlod as u64, ylod as u64, zlod as u64, lod);
    	//println!("{},{},{} ",x,y,z);
    	//println!("{},{},{} morton: {}",xlod, ylod, zlod, m);
    	//println!("voxels: \n {}", self.lod_voxels[index as usize]);
    	let isset = self.lod_voxels[index as usize].isset(m);
    	//println!("isset: {}", isset);
    	isset
    }
    
    //recursively checks at low LOD first if it hits, proceeds to the highest detail if all lowe level LOD's are hit
    pub fn hit_optimize(&self, x:i64, y:i64, z:i64)->bool{
    	let mut hit_counter = 0;
    	for detail in range(1, self.lod){
    		let hit = self.hit_at_lod(x, y, z, detail);
    		if !hit {
    			return false;
    		}
    		if hit {
    			hit_counter += 1;
    		}
    	}
    	//println!("hit counter: {}", hit_counter);
    	if hit_counter == self.lod -1 {
    		//println!("all lower LOD are hit.. trying the highest detail..");
    		return self.hit_direct(x,y,z);
    	}
    	true
    }
    
    pub fn get_color(&self, x:i64, y:i64, z:i64)->Color{
        let m = morton(x as u64, y as u64, z as u64, self.lod);
        self.voxel.get_color(m)
    }
    
    //get the x,y,z at given lod
    fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	//println!("current lod: {} --> {}", self.lod, new_lod);
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64).round() as i64;
    	//println!("new {},{},{}",xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    //build the voxel LOD's at each level of detail
    pub fn build_voxel_lod(&mut self){
    	 let mut parent_voxel = self.voxel.clone();
    	 for i in range(0, self.lod){
    	 	//println!("Building voxel at LOD: {}", i);
    		parent_voxel = parent_voxel.parent();
    		//parent_voxel.display_bitset();
    		self.lod_voxels.push(parent_voxel.clone());
    		
    	}
    }
    
    
 }  

 


 





pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){

	let mut file = File::create(&Path::new(filename));
	let header = String::from_str(format!("P6\n# CREATOR: lee\n").as_slice());
	file.write(header.into_bytes().as_slice());

	let size = String::from_str(format!("{} {}\n255\n", width, height).as_slice());
	file.write(size.into_bytes().as_slice());

	for p in range(0,pixels.len()){
		file.write_u8(pixels[p].r);
		file.write_u8(pixels[p].g);
		file.write_u8(pixels[p].b);
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
