extern crate time;
extern crate glutin;

use std::num::Float;
use std::collections::BTreeSet;

use voxel::Voxel;
use ray::Ray;
use vector::Vector;
use point::Point;
use screen::Screen;
use color::Color;
use morton::morton;
use time::get_time;

use std::old_io::File;
use voxelizer::Voxelizer;

use time::Timespec;

mod voxel;
mod ray;
mod vector;
mod point;
mod screen;
mod color;
mod morton;
mod voxelizer;

fn main(){
	let lod:u8 = 8;
    let limit:u64 = 1 << lod;
    let r:u64 = 1 << lod-1;//do a radius of half the limit
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    let max_distance = 2 * (((limit * limit * limit ) as f64).sqrt().round()) as u64 + 1;
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
    
    //look at the center of the sphere
    let xlookat = (limit/2) as i64;
    let ylookat = (limit/2) as i64;
    let zlookat = (limit/2) as i64;

	//put the camera away from sphere in z direction, slightly up and slightly right
    let xcam = (limit/2) as i64;
    let ycam = (limit/2) as i64;
    let zcam = -3 * (limit) as i64;

    let xcam = -(limit) as i64;
    let ycam = -(limit) as i64;
    let zcam = -(limit) as i64;
    
    let camera = Point{x:xcam, y:ycam, z:zcam};
    let lookat = Point{x:xlookat, y:ylookat, z:zlookat};
    println!("camera location: {}", camera);
    println!("looking at: {}", lookat);

	//distance
	
	
    let width = 800;
    let height = 800;
    let fd = width/2;
    let screen = Screen::new(width, height, fd, lod);
    
    let total = width * height;
    
	let mut pixels:Vec<Color> =Vec::new();
	for t in range(0, total){
	    pixels.push(Color{r:255,g:255,b:255});
	}
    
    //transform the direction here
	//compute the pitch, yaw, roll
	let pitch = ((lookat.y - camera.y) as f64/(lookat.z - camera.z) as f64).atan();//along x
	let yaw =   ((lookat.x - camera.x) as f64/(lookat.z - camera.z) as f64).atan();//along y
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    println!("Tracing...");
    let mut cnt = 0;
    let mut percentage = 0;
    let mut took_counter = 0;
    let mut took_total = 0.0;
    let mut max_took = 0.0;
    for iy in range(0, height){
        let new_percentage = ((iy as f64/height as f64) * 100.0).round() as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;
		for jx in range(0,width){
	        let t1:Timespec = get_time();
			let mut pixel_screen:Vector = screen.at_pixel(jx, iy);//direction of pixel relative to the screen facing forward
			let rotated_pixel_screen = pixel_screen.rotate_at_y(yaw);
			let rotated_at_pitch = rotated_pixel_screen.rotate_at_x(-pitch);
			let final_location = rotated_at_pitch.add_vector(pixel_screen.clone()).add(camera.clone());
			let pixel_screen_camera = rotated_at_pitch.add(camera.clone());//the most correct when no rotation
			
			let pixel_ray = Ray::new(camera.clone(), pixel_screen_camera, lod);
			//let pixel_ray = Ray::new(camera.clone(), final_location, lod);
			
			let mut length = 0;
			let index = iy * width + jx;
			loop {
				let point = pixel_ray.at_length(length);
				let hit = voxelizer.hit_optimize(point.x, point.y, point.z);
				if hit {
					pixels[index as usize] = voxelizer.get_color(point.x, point.y, point.z);
					break;
				}
				if length >= max_distance {
					break;
				}
				length += 1;
			}
			cnt += 1;
			let t2:Timespec = get_time();
			let took:f64 = (t2.nsec as f64 - t1.nsec as f64) / 1_000_000.0;
			//if cnt % 100 == 0 { println!("took: {} ms", took);}
			if took > max_took{
				max_took = took;
			}
			if took >= 0.0 {
				took_counter+=1;
				took_total += took;
			}
		}
		
    }
    println!("took average: {} ms for each pixel", took_total/took_counter as f64);
    println!("max took: {}", max_took);
    
    let filename = String::from_str(format!("pic{}.ppm",lod).as_slice());
    println!("filename: {}",filename);
    voxelizer::save_to_file(filename, pixels, width, height);
}
