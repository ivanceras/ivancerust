use std::num::Float;
use std::collections::BTreeSet;

use voxel::Voxel;
use ray::Ray;
use vector::Vector;
use point::Point;
use screen::Screen;
use color::Color;

use std::io::File;

mod voxel;
mod ray;
mod vector;
mod point;
mod screen;
mod color;

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
    
    voxel:Voxel, //the highest resolution voxel
    
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
                    //println!("inside: {}, {}, {} morton: {}",i,j,k, m);
                  }
                  
                  else if sqrt_iijjkk < self.r as f64 {
                    self.inside += 1;
                    self.voxel.set_bit(m, true);
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
    		println!("{} {} {} not bounded..",x, y, z);
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
    fn hit_optimize(&self, x:i64, y:i64, z:i64)->bool{
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
    fn build_voxel_lod(&mut self){
    	 let mut parent_voxel = self.voxel.clone();
    	 for i in range(0, self.lod){
    	 	//println!("Building voxel at LOD: {}", i);
    		parent_voxel = parent_voxel.parent();
    		//parent_voxel.display_bitset();
    		self.lod_voxels.push(parent_voxel.clone());
    		
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
	let lod:u8 = 9;
    let limit:u64 = 1 << lod;
    let r:u64 = 1 << lod-1;//do a radius of half the limit
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    let max_distance = ((limit * limit * limit ) as f64).sqrt().round() as u64;
    println!("max distance: {}", max_distance);
    voxelizer.start();
    //voxelizer.debug();
    //voxelizer.voxel.show_indexes();
    voxelizer.build_voxel_lod();
    
    
    
    println!("Displaying voxels at all levels..");
    let levels = voxelizer.lod_voxels.len();
    println!("levels: {}", levels);
    for lev in range (0, levels){
    	let actual_lod = lod - (lev+1) as u8;
    	println!("at actual lod: {}",actual_lod);
    	println!("{}",voxelizer.lod_voxels[lev]);
    }
    
    //look at the center of the sphere
    let cx = (limit/2) as i64;
    let cy = (limit/2) as i64;
    let cz = (limit/2) as i64;

	//put the camera away from sphere in z direction, slightly up and slightly right
    let xorig = (limit as i64/2) + 10;
    let yorig = (limit as i64/2) + 10;
    let zorig = -(limit as i64);
    
    println!("origin: {}, {}, {}", xorig, yorig, zorig);
    println!("looking at: {}, {}, {}", cx, cy, cz);

	//distance
	let dx = (cx - xorig) as f64;
	let dy = (cy - yorig) as f64;
	let dz = (cz - zorig) as f64;
	
	let v = Vector{x:dx, y:dy, z:dz};
	
	println!("vector: {}",v);
	
    let r = Ray::new(xorig, yorig, zorig, v, lod);//this is the center ray, base on this ray compute the other ray at the sides
    let mut p = 0;
	

    loop {
    	let point = r.at_length(p);
		let hit = voxelizer.hit_optimize(point.x, point.y, point.z);
		//let hit_lod = voxelizer.hit_at_lod(point.x, point.y, point.z, 4);
		if hit {
			println!("hit at: {}, {}, {} --> {} ", point.x, point.y, point.z, hit);
    		break;
    	}
    	p += 1;
    }
    let vx = dx;//v.x;
    let vy = dy;//v.y;
    let vz = dz;//v.z;
    
    let width = 800;
    let height = 600;
    let fd = width/2;
    let screen = Screen::new(width, height, vx, vy, vz, fd);
    screen.compute_rays();
    let r00 = screen.at_pixel(0,0);
    println!("ray00: {}",r00);
    println!("center: {}",screen.at_pixel(50,50));
    
    
    let total = width * height;
	println!("total: {}", total);
	let mut pixels:Vec<Color> =Vec::new();
	for t in range(0, total){
	    pixels.push(Color{r:255,g:255,b:255});
	}
    
    let mut cnt = 0;
    for iy in range(0, height){
		for jx in range(0,width){
			let pixel_vector = screen.at_pixel(jx, iy);
			//println!("pixel vector: {}",pixel_vector);
			let pixel_ray = Ray::new(xorig, yorig, zorig, pixel_vector, lod);
			let mut length = 0;
			let index = iy * width + jx;
			println!("index: {} cnt: {}", index,cnt);
			loop {
				let point = pixel_ray.at_length(length);
				//println!("point: {}", point);
				let hit = voxelizer.hit_optimize(point.x, point.y, point.z);
				if hit {
					pixels[index as usize] = Color{r:0,g:0,b:0};
					break;
				}
				if length >= max_distance {
					//pixels[index as usize] = Color{r:255,g:255,b:255};
					break;
				}
				length += 1;
			}
			cnt += 1;
		}
    }
    
    save_to_file(pixels, width, height);
}



fn save_to_file(pixels:Vec<Color>, width:i64, height:i64){

	let mut file = File::create(&Path::new("pic.ppm"));
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
