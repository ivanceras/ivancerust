extern crate time;
extern crate glutin;

use std::num::Float;
use std::collections::BTreeSet;
use std::num::SignedInt;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::sync::mpsc;

use voxel::Voxel;
//use indexed_voxel::Voxel;

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

//mod indexed_voxel;
mod voxel;

mod ray;
mod vector;
mod point;
mod screen;
mod color;
mod morton;
mod voxelizer;

fn main(){

	//let file_pre = "./sphere/pic";//sphere
	let file_pre = "./pic";//cubes
	let lod:u8 = 5;//lod of the object when voxelizing
    let limit:u64 = 1 << lod;

	let view_lod = 8;//base LOD at screen 1 voxel = 1 pixel
    let view_limit = 1 << view_lod;

    let r:u64 = 1 << lod-1;//do a radius of half the limit
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    
   
    let voxelizing_t1  = get_time();
    println!("Voxelizing..");
    voxelizer.start();
    voxelizer.build_voxel_lod();
    println!("Done building voxel data...");
    let voxelizing_t2  = get_time();
    let voxelizing_took = (voxelizing_t2.nsec as f64 - voxelizing_t1.nsec as f64) / 1_000_000.0;
    println!("voxelizing took: {} ms",voxelizing_took);
    let levels = voxelizer.lod_voxels.len();
    println!("levels: {}", levels);
    
    //look at the center of the sphere
    let xlookat = (limit/2) as i64;
    let ylookat = (limit/2) as i64;
    let zlookat = (limit/2) as i64;

	//put the camera away from sphere in z direction, slightly up and slightly right

    let xcam = -(limit) as i64;
    let ycam = -(limit) as i64;
    let zcam = -(limit) as i64;
    
    let camera = Point{x:xcam, y:ycam, z:zcam};
    let lookat = Point{x:xlookat, y:ylookat, z:zlookat};
    println!("camera location: {}", camera);
    println!("looking at: {}", lookat);
    
    let max_distance = 2 * view_limit as u64 + zcam.abs() as u64;
 	println!("max distance: {}", max_distance);
	
	
    let width = 800;
    let height = 800;
    let fd = width/2;
    let screen = Screen::new(width, height, fd, view_lod);
    
    let total = width * height;
    
	let mut pixels:Vec<Color> =Vec::new();
	for t in range(0, total){
	    pixels.push(Color{r:255,g:255,b:255});//white background
	}
    
    //transform the direction here
	//compute the pitch, yaw, roll
	let pitch = ((lookat.y - camera.y) as f64/(lookat.z - camera.z) as f64).atan();//along x
	let yaw =   ((lookat.x - camera.x) as f64/(lookat.z - camera.z) as f64).atan();//along y
	//let pitch = (30.0).to_radians();//along x
	//let yaw =   (10.0).to_radians();//along y
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    println!("Tracing...");
    let tracing_t1 = get_time();
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
            let pixel_screen_camera = rotated_at_pitch.add(camera.clone());//the most correct when no rotation
            let pixel_ray = Ray::new(camera.clone(), pixel_screen_camera, view_lod);
            
            let color = voxelizer.trace(pixel_ray, max_distance);//get the color of the voxel that is hit at this direction
            let index = iy * width + jx;
            pixels[index as usize] = color;
            cnt += 1;
            let t2:Timespec = get_time();
            let took:f64 = (t2.nsec as f64 - t1.nsec as f64) / 1_000_000.0;
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
    let tracing_t2 = get_time();
    let tracing_took = (tracing_t2.nsec as f64 - tracing_t1.nsec as f64) / 1_000_000.0;
    println!("Tracing took: {} ms",tracing_took);
    
    let filename = String::from_str(format!("{}{}.ppm",file_pre,lod).as_slice());
    println!("filename: {}",filename);
    voxelizer::save_to_file(filename, pixels, width, height);
}


