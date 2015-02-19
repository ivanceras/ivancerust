use std::num::Float;
use std::collections::BTreeSet;

use voxel::Voxel;
use ray::Ray;
use vector::Vector;
use point::Point;
use screen::Screen;
use color::Color;
use morton;
use std::thread::Thread;

use std::old_io::File;

pub struct Voxelizer{
	lod:u8,
    limit:u64,
    r:u64,
    voxel:Voxel, //the highest resolution voxel
	lod_voxels:Vec<Voxel>, //voxels with the LOD
}

impl Voxelizer{

    pub fn new(lod:u8, limit:u64, r:u64)->Voxelizer{
        Voxelizer{
            lod:lod, 
            limit:limit, 
            r:r, 
            voxel:Voxel::new(lod),
            lod_voxels:Vec::new()
            
          }
    }
    
    pub fn start(&mut self){
    	let xlimit = self.limit;
        let ylimit = self.limit;
        let zlimit = self.limit;
        
        let cx = (xlimit/2) as i64;
        let cy = (ylimit/2) as i64;
        let cz = (zlimit/2) as i64;
        
        let mut percentage = 0;
        for i in range (0, xlimit){
            let new_percentage = ((i as f64/xlimit as f64) * 100.0).round() as u64;
            if new_percentage != percentage {
                println!("{} %", percentage);
            }
            percentage = new_percentage;
            for j in range (0, ylimit) {
                for k in range (0, zlimit){
                      //sign matters here
                      let x = (i as i64 - cx);
                      let y = (j as i64 - cy);
                      let z = (k as i64 - cz);
                      if self.is_inside_cube(x, y, z){
		                  let index = i * ylimit * zlimit + j * zlimit + k;
		                  let m = morton::encode(i, j, k, self.lod);
		                  let r = 256 - ((i as f64 / self.limit as f64) * 256.0).round() as u8;
						  let g = 256 - ((j as f64 / self.limit as f64) * 256.0).round() as u8;
						  let b = 256 - ((k as f64 / self.limit as f64) * 256.0).round() as u8;
		                  let color = Color::new(r,g,b,255);
		                  self.voxel.set_bit_at_loc(i, j, k, true, color);
                      }
                  }
               }    
            }
    }
    
    //is inside sphere
	fn is_inside_sphere(&self, x:i64, y:i64,z:i64)->bool{
		let xf = x as f64;
		let yf = y as f64;
		let zf = z as f64;
		let rad = (xf*xf + yf*yf + zf*zf).sqrt().round() as u64;
		if rad <= self.r {
			return true;
		}
		false
	}
	
	//is inside cube, r is the half the length of side
	fn is_inside_cube(&self, x:i64, y:i64,z:i64)->bool{
		let len = self.r as i64;
		if x >= -len && x <= len
		  && y >= -len && y <= len
		  && z >= -len && z <= len {
			return true;
		}
		false
	}
    
    
     //determines if the point is inside the boundary of this voxel
    fn bounded(&self, x:i64, y:i64, z:i64)->bool{
    	let xlowerbound = 0;
    	let ylowerbound = 0;
    	let zlowerbound = 0;
 		let xupperbound = self.limit as i64;
 		let yupperbound = self.limit as i64;
 		let zupperbound = self.limit as i64;
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
    		return false;
    	}
    	let m = morton::encode(x as u64, y as u64, z as u64, self.lod);
    	let isset = self.voxel.isset(m);
    	isset
    }
    
    //check if point at a certain LOD (level of detail) hits
     fn hit_at_lod(&self, x:i64, y:i64, z:i64, lod:u8)->bool{
     	let bounded = self.bounded(x, y, z);
     	if !bounded {
    		return false;
    	}
    	let (xlod, ylod, zlod) = self.at_lod(x, y, z, lod);
    	//let index = self.lod - (lod+1);
    	let m = morton::encode(xlod as u64, ylod as u64, zlod as u64, lod);
    	//println!("lod: {}, self.lod_voxels.len(): {}, index: {}",lod, self.lod_voxels.len(), index);
    	let isset = self.lod_voxels[lod as usize].isset(m);
    	isset
    }
    
    //recursively checks at low LOD first if it hits, proceeds to the highest detail if all lowe level LOD's are hit
    pub fn hit_optimize(&self, x:i64, y:i64, z:i64)->bool{
    	let mut hit_counter = 0;
    	for detail in range(1, self.lod){//1 to 4
    		let hit = self.hit_at_lod(x, y, z, detail);//detail is the index of the LOD list, 1 is the lowest LOD
    		if !hit {
    			return false;
    		}
    		else {
    			hit_counter += 1;
    		}
    	}
    	if hit_counter == self.lod -1 {
    		return self.hit_direct(x,y,z);
    	}
    	false
    }
    
    pub fn get_color(&self, x:i64, y:i64, z:i64)->Color{
        let m = morton::encode(x as u64, y as u64, z as u64, self.lod);
        self.voxel.get_color(m)
    }
    
    //get the x,y,z at given lod
    fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64).round() as i64;
    	//println!("at lod {}: {},{},{}", new_lod, xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    //build the voxel LOD's at each level of detail
    pub fn build_voxel_lod(&mut self){
    	 let mut parent_voxel = self.voxel.clone();
    	 for i in range(0, self.lod){
    		self.lod_voxels.push(Voxel::new(i+1));
    		
    	}
    	 for i in range(0, self.lod){
    	 	println!("Building voxel at LOD: {}", i);
    		parent_voxel = parent_voxel.parent();
    		//println!("voxels: {}", parent_voxel);
    		let index = ((self.lod - i) -1) as usize;
    		self.lod_voxels[index]= parent_voxel.clone();
    		
    	}
    }
    
    //trace the voxels at pixel_ray direction, marching the distance up to max_distance
    pub fn trace(&self, pixel_ray:Ray, max_distance:u64)->Color{
        let mut length = 0;
        loop {
            let point = pixel_ray.at_length(length);
            //let hit = self.hit_optimize(point.x, point.y, point.z);
            let hit = self.hit_direct(point.x, point.y, point.z);
            if hit {
                return self.get_color(point.x, point.y, point.z)
            }
            if length >= max_distance {
                return Color::new(255,255,255,255);
            }
            length += 1;
        }
    }
    
    
 }  

 
pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){

	let mut file = File::create(&Path::new(filename));
	let header = String::from_str(format!("P6\n# CREATOR: lee\n").as_slice());
	file.write(header.into_bytes().as_slice());

	let size = String::from_str(format!("{} {}\n255\n", width, height).as_slice());
	file.write(size.into_bytes().as_slice());
	let mut buffer = Vec::new();
	for p in range(0,pixels.len()){
		buffer.push(pixels[p].r);
		buffer.push(pixels[p].g);
		buffer.push(pixels[p].b);
	}
	file.write_all(buffer.as_slice());
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
