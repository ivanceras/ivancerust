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
use voxelizer::Voxelizer;

mod voxel;
mod ray;
mod vector;
mod point;
mod screen;
mod color;
mod morton;
mod voxelizer;

fn main(){
	let lod:u8 = 5;
    let limit:u64 = 1 << lod;
    let r:u64 = 1 << lod-1;//do a radius of half the limit
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    let max_distance = (((limit * limit * limit ) as f64).sqrt().round()) as u64 + 1;
    //let max_distance = 10000;// if encounters nothing after 10,000 march, then just break (far clipping space)
    println!("max distance: {}", max_distance);
    println!("Voxelizing..");
    voxelizer.start();
    //voxelizer.debug();
    //voxelizer.voxel.show_indexes();
    voxelizer.build_voxel_lod();
    println!("Done building voxel data...");
    
    
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
    let xorig = (limit as i64/2);
    let yorig = (limit as i64/2);
    let zorig = -(limit as i64/2);
    
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
    let height = 800;
    let fd = width/2;
    let screen = Screen::new(width, height, vx, vy, vz, fd);
    //screen.compute_rays();
    let r00 = screen.at_pixel(0,0);
    println!("ray00: {}",r00);
    println!("center: {}",screen.at_pixel(width/2, height/2));
    
    
    let total = width * height;
	println!("total: {}", total);
	let mut pixels:Vec<Color> =Vec::new();
	for t in range(0, total){
	    pixels.push(Color{r:255,g:255,b:255});
	}
    
    println!("Tracing...");
    let mut cnt = 0;
    let mut percentage = 0;
    for iy in range(0, height){
        let new_percentage = ((iy as f64/height as f64) * 100.0).round() as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;
		for jx in range(0,width){
			let pixel_vector = screen.at_pixel(jx, iy);
			//println!("pixel vector: {}",pixel_vector);
			let pixel_ray = Ray::new(xorig, yorig, zorig, pixel_vector, lod);
			let mut length = 0;
			let index = iy * width + jx;
			//println!("index: {} cnt: {}", index,cnt);
			loop {
				let point = pixel_ray.at_length(length);
				//println!("point: {}", point);
				let hit = voxelizer.hit_optimize(point.x, point.y, point.z);
				if hit {
					pixels[index as usize] = voxelizer.get_color(point.x, point.y, point.z);
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
    
    let filename = String::from_str(format!("pic{}.ppm",lod).as_slice());
    println!("filename: {}",filename);
    voxelizer::save_to_file(filename, pixels, width, height);
}
