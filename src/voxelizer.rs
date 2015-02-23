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
use shape::Sphere;
use shape::Shape;


pub struct Voxelizer{
	lod:u8,
    limit:u64,
    voxel:Voxel, //the highest resolution voxel
	lod_voxels:Vec<Voxel>, //voxels with the LOD
}

impl Voxelizer{

    pub fn new(lod:u8, limit:u64, r:u64)->Voxelizer{
        Voxelizer{
            lod:lod, 
            limit:limit, 
            voxel:Voxel::new(lod),
            lod_voxels:Vec::new()
          }
    }
    
    //start to voxelize objects
    pub fn start<T: Shape>(&mut self, shape:T){
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
	                  let point = Point::new(i as i64, j as i64, k as i64);
                      if shape.is_inside(&point){
		                  let r = 256 - ((i as f64 / self.limit as f64) * 255.0).round() as u8;
						  let g = 256 - ((j as f64 / self.limit as f64) * 255.0).round() as u8;
						  let b = 256 - ((k as f64 / self.limit as f64) * 255.0).round() as u8;
		                  let color = Color::new(r,g,b,255);
		                  self.voxel.set_bit_at_loc(i, j, k, true, color);
		                  //let normal = shape.normal(&point);
                      }
                      
                  }
               }    
            }
    }
    
    //check if point at a certain LOD (level of detail) hits
     fn hit_at_lod(&self, x:i64, y:i64, z:i64, lod:u8)->bool{
        let lod_voxel = &self.lod_voxels[lod as usize];
    	let (xlod, ylod, zlod) = self.voxel.at_lod(x, y, z, lod);
    	let isset = lod_voxel.hit(xlod, ylod, zlod);
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
    		return self.voxel.hit(x,y,z);
    	}
    	false
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
    //the optimum version will traverse the voxel at higer level LOD, go to higer LOD only when a hit is made
    pub fn trace(&self, pixel_ray:Ray, max_distance:u64, scale:f64)->Color{
        let mut length = 0;
        loop {
            let point = pixel_ray.at_length(length);
            let px = (point.x / scale ).round() as i64;
            let py = (point.y / scale ).round() as i64;
            let pz = (point.z / scale ).round() as i64;
            //let hit = self.hit_optimize(px, py, pz);
            let hit = self.voxel.hit(px, py, pz);
            if hit {
                return self.voxel.get_color_at_loc(px, py, pz)
            }
            if length >= max_distance {
                return Color::new(255,255,255,255);
            }
            length += 1;
        }
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
