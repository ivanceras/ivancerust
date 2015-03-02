extern crate time;

use std::num::Float;
use std::num::SignedInt;
use std::sync::Arc;
use std::thread::Thread;
use std::sync::mpsc;
use std::old_io::File;


use ray::Ray;
use vector::Vector;
use point::Point;
use screen::Screen;
use color::Color;
use voxelizer::Voxelizer;
use shape::Sphere;
use shape::Cube;
use shape::Shape;


pub fn render(voxelizer:Voxelizer, trace_lod:u8, view_lod:u8, lookat:Point, camera:&Point, width:i64, height:i64, focal_distance:i64)->Vec<Color>{
    let view_limit = 1 << view_lod;
    println!("side voxels: {}",view_limit);
    
    let max_distance = 4 * view_limit as u64 + camera.z.abs() as u64;
 	println!("max distance: {}", max_distance);
	
	
    let screen = Screen::new(width, height, focal_distance, view_lod);
	let pitch = ((lookat.y - camera.y) as f64/(lookat.z - camera.z) as f64).atan();//along x
	let yaw =   ((lookat.x - camera.x) as f64/(lookat.z - camera.z) as f64).atan();//along y
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    let mut cnt = 0;
    let mut percentage = 0;
    render_threaded(voxelizer, pitch, yaw, camera, screen, 1.0 , max_distance, trace_lod, view_lod)

}

//render a scene using multi threading via spawn
//trace_lod, when trace_lod is reached the ray is stopped and the color is returned
fn render_threaded(voxelizer:Voxelizer, pitch:f64, yaw:f64, camera:&Point, screen:Screen, scale:f64, max_distance:u64, trace_lod:u8, view_lod:u8)->Vec<Color>{
    let mut pixels:Vec<Color> = Vec::new();
    let total = screen.width * screen.height;
	for t in range(0, total){
	    pixels.push(Color::black());//white background
	}
	
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    println!("Threaded Tracing...");
    let mut cnt = 0;
    let mut percentage = 0;
    let (tx, rx) = mpsc::channel();
	
	let arc_voxelizer = Arc::new(voxelizer);
	let mut total_pixel_took = 0;
	let wcenter = screen.width/2;
	let hcenter = screen.height/2;
    for iy in 0..screen.height {
        let new_percentage = ((iy as f64/screen.height as f64) * 100.0).round() as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;

        for jx in 0..screen.width {
            let mut pixel_screen:Vector = screen.at_pixel(jx, iy);//direction of pixel relative to the screen facing forward
            let rotated_pixel_screen = pixel_screen.rotate_at_y(yaw);
            let rotated_at_pitch = rotated_pixel_screen.rotate_at_x(-pitch);
            let pixel_screen_camera = rotated_at_pitch.add_point(&camera);//the most correct when no rotation
            let pixel_ray = Ray::new(&camera, pixel_screen_camera);
            
            let index = iy * screen.width + jx;
	        ///////////////////////
	        // Start of Threading
	        ///////////////////////
	        
            let tx_clone = tx.clone();
			let arc_voxelizer_clone = arc_voxelizer.clone();
	        Thread::spawn(move || {
	        	 let mut color = arc_voxelizer_clone.cone_trace(pixel_ray, max_distance, scale, trace_lod, view_lod);//get the color of the voxel that is hit at this direction
	        	 if iy == hcenter && jx == wcenter {
	        	    color = Color::purple();//mark the center a different color (purple)
	        	 }
	        	 tx_clone.send((index,color));
	        });
            /////////////////////
	        // End of Threading
	        //////////////////////
	        
	        //let color = voxelizer.trace(pixel_ray, max_distance);
            //pixels[index as usize] = color;
            cnt += 1;
        }
      
    }
   	let mut total_pixel_tracing_ns = 0;
    //////////////////////////////////
    // Start of Receiving thread result
    ////////////////////////////////////

	for ir in 0..screen.height{
	    for jr in range(0, screen.width){
	    	let (index,color) = rx.recv().ok().expect("Could not recieve answer");
	   		pixels[index as usize] = color;
	    }
	}
	return pixels;
}


//save pixels to file
pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){
	let mut file = File::create(&Path::new(filename));
	let header = String::from_str(format!("P6\n# CREATOR: lee\n").as_slice());
	let size = String::from_str(format!("{} {}\n255\n", width, height).as_slice());

	let mut buffer = Vec::new();
    buffer.push_all(header.into_bytes().as_slice());
    buffer.push_all(size.into_bytes().as_slice());
    
	for p in 0..pixels.len() {
		buffer.push(pixels[p].r);
		buffer.push(pixels[p].g);
		buffer.push(pixels[p].b);
	}
	file.write_all(buffer.as_slice());
}

