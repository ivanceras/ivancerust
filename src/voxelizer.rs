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

    pub fn new(lod:u8)->Voxelizer{
        Voxelizer{
            lod:lod, 
            limit:1<<lod, 
            voxel:Voxel::new(lod),
            lod_voxels:Vec::new()
          }
    }
    
    pub fn set_voxel(&mut self, voxel:Voxel){
		self.voxel = voxel;
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
                println!("{} %", new_percentage);
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
    	let lod_index = (lod - 1) as usize;
        let lod_voxel = &self.lod_voxels[lod_index];
    	let (xlod, ylod, zlod) = lod_voxel.at_lod(x, y, z, lod);//calculated at voxel LOD level (same level)
    	let isset = lod_voxel.hit(xlod, ylod, zlod);//no bound checking
    	isset
    }
    
    //build voxel against higest LOD
    /*
    pub fn build_voxel_lod(&mut self){
    	 for i in range(0, self.lod-1){
    	 	let new_lod = i+1;
    	 	println!("Building voxel at new lod: {}", new_lod);
    	 	let subvoxel = self.voxel.build_voxel_lod(new_lod);
    	 	//println!("sub voxel: {}", subvoxel);
    	 	self.lod_voxels.push(subvoxel);
    	}
    }
    */
    
    //build voxel against their intermediate higher level LOD
    //bottom up
     pub fn build_voxel_lod(&mut self){
     	let mut subvoxel = self.voxel.clone();
     	for i in range(0, self.lod-1){
    	 	let new_lod = i+1;
    	 	self.lod_voxels.push(Voxel::new(new_lod));
	 	}
    	
    	for i in range(0, self.lod-1){
    		let new_lod = self.lod - (i+1);
    	 	let index = new_lod - 1;
    	 	println!("Building voxel at new lod: {}, index:{}", new_lod, index);
    	 	subvoxel = subvoxel.build_voxel_lod(new_lod);
    	 	self.lod_voxels[index as usize] = subvoxel.clone();
    	}
    }
	
	//build only at this LOD level
	pub fn build_voxel_lod_level(&mut self, level:u8){
		if level < self.lod {
	     	for i in range(0, self.lod-1){
	    	 	let new_lod = i+1;
	    	 	self.lod_voxels.push(Voxel::new(new_lod));
		 	}
		 	let index = level - 1;
		 	println!("Building voxel at new lod: {}, index:{}", level, index);
		 	self.lod_voxels[index as usize] = self.voxel.build_voxel_lod(level);
	 	}
    }
    
    //trace the voxels at pixel_ray direction, marching the distance up to max_distance
    //the optimum version will traverse the voxel at higer level LOD, go to higer LOD only when a hit is made
    pub fn trace(&self, pixel_ray:Ray, max_distance:u64, scale:f64, max_lod:u8)->Color{
        let mut length = 0;
        loop {
            let point = pixel_ray.at_length(length);
            let px = (point.x / scale ).round() as i64;
            let py = (point.y / scale ).round() as i64;
            let pz = (point.z / scale ).round() as i64;
            //println!("tracing.. point: ({}, {}, {})",px,py,pz);
            //let hit = self.hit_optimize(px, py, pz);
            let hit = self.voxel.hit(px, py, pz);
            if hit {
                return self.voxel.get_color_at_loc(px, py, pz)
            }
            if length >= max_distance {
                return Color::white();
            }
            length += 1;
        }
    }

    
    pub fn cone_trace(&self, pixel_ray:Ray, max_distance:u64, scale:f64, trace_lod:u8, view_lod:u8)->Color{
        let mut length_increment = 1;
        //let view_lod = 6; //really depends on the screen resolution
        let voxel_scale = 1 << (view_lod - trace_lod);//current_lod should be lesser than view_lod
        //let voxel_scale = 1 << (view_lod - self.lod);
        assert!(trace_lod <= view_lod, "view should be finer than the voxel being traced");//view should be finer than the voxel being traced

        let trace_lod_index = (trace_lod - 1) as usize;
        let mut length = 0;
        let highest = trace_lod == self.lod;//if trace_lod == self.lod use the highest LOD voxel
        loop {
            let point = pixel_ray.at_length(length);
            let px = (point.x / (scale * voxel_scale as f64) ).round() as i64;
            let py = (point.y / (scale * voxel_scale as f64) ).round() as i64;
            let pz = (point.z / (scale * voxel_scale as f64) ).round() as i64;
            let hit = if highest {self.voxel.hit(px, py, pz)} else {self.hit_at_lod(px, py, pz, trace_lod)};
            if hit {
            	let lod_voxel = if highest {&self.voxel} else {&self.lod_voxels[trace_lod_index]};
            	let color =  lod_voxel.get_color_at_loc(px, py, pz);
            	return color;
            }
            if length >= max_distance {
                return Color::white();
            }
            length += length_increment;
        }
    }
 }  

