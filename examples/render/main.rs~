extern crate ivancerust;
extern crate time;

use std::num::Float;
use std::num::SignedInt;
use std::sync::Arc;
use std::thread::Thread;
use std::sync::mpsc;
use std::old_io::File;
use time::PreciseTime;


use ivancerust::ray::Ray;
use ivancerust::vector::Vector;
use ivancerust::point::Point;
use ivancerust::screen::Screen;
use ivancerust::color::Color;
use ivancerust::voxelizer::Voxelizer;
use ivancerust::shape::Sphere;
use ivancerust::shape::Cube;
use ivancerust::shape::Shape;


fn main(){
	//let use_threads = true;
	let file_pre = "./target/renders/";//cubes
	let lod:u8 = 5;//lod of the object when voxelizing
	let max_lod = lod;
    let limit:u64 = 1 << lod;
    let scale = 1.0;

	let view_lod = 8;//base LOD at screen 1 voxel = 1 pixel
    let view_limit = 1 << view_lod;
    println!("side voxels: {}",view_limit);

    let r:u64 = 1 << lod-1;//do a radius of half the limit
   
    let voxelizing_t1  = PreciseTime::now();
    
    let cx = (limit/2) as i64;
    let cy = (limit/2) as i64;
    let cz = (limit/2) as i64;
    
    let center = Point::new(cx, cy, cz);
    let shape = Sphere::new(r, &center);
    //let shape = Cube::new(r, &center);
    
    let filename = String::from_str(format!("{}{}{}.ppm",file_pre,shape.name(),lod).as_slice());

    println!("Voxelizing..{} lod: {}",shape.name(),lod);
    let mut voxelizer = Voxelizer::new(lod, limit, r);
    voxelizer.start(shape);
    voxelizer.build_voxel_lod();
    println!("Done building voxel data...");
    let voxelizing_took = voxelizing_t1.to(PreciseTime::now()).num_milliseconds();
    println!("voxelizing took: {} ms",voxelizing_took);	
    //let levels = voxelizer.lod_voxels.len();
    //println!("levels: {}", levels);
    
    //look at the center of the sphere
    let xlookat = (limit/2) as i64;
    let ylookat = (limit/2) as i64;
    let zlookat = (limit/2) as i64;

	//put the camera away from sphere in z direction, slightly up and slightly right
    //let xcam = -(limit as f64 * scale / 2.0) as i64;
    //let ycam = -(limit as f64 * scale / 2.0) as i64;
    //let zcam = -(limit as f64 * scale / 2.0) as i64;

    let xcam = -50;
    let ycam = -50;
    let zcam = -50;
    
    let camera = Point{x:xcam, y:ycam, z:zcam};
    let lookat = Point{x:xlookat, y:ylookat, z:zlookat};
    println!("camera location: {}", camera);
    println!("looking at: {}", lookat);
    
    let max_distance = 4 * view_limit as u64 + zcam.abs() as u64;
 	println!("max distance: {}", max_distance);
	
	
    let width = 800;
    let height = 800;
    let fd = width/2;
    let screen = Screen::new(width, height, fd, view_lod);
	let pitch = ((lookat.y - camera.y) as f64/(lookat.z - camera.z) as f64).atan();//along x
	let yaw =   ((lookat.x - camera.x) as f64/(lookat.z - camera.z) as f64).atan();//along y
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    let mut cnt = 0;
    let mut percentage = 0;
    let pt1 = PreciseTime::now();
    //let pixels = if use_threads {
    //    render_threaded(voxelizer, pitch, yaw, camera, screen, scale, max_distance, max_lod)
    //}
    //else{
    //    render(voxelizer, pitch, yaw, camera, screen, scale, max_distance, max_lod)
    //};
    let pixels = render_threaded(voxelizer, pitch, yaw, camera, screen, scale, max_distance, max_lod);
    let rendering_duration = pt1.to(PreciseTime::now());
    println!("rendering time: {} ms",rendering_duration.num_milliseconds());
   
    println!("filename: {}",filename);
    save_to_file(filename, pixels, width, height);

}

//render a scene using multi threading via spawn
//max_lod, when max_lod is reached the ray is stopped and the color is returned
fn render_threaded(voxelizer:Voxelizer, pitch:f64, yaw:f64, camera:Point, screen:Screen, scale:f64, max_distance:u64, max_lod:u8)->Vec<Color>{
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
	let pt1 = PreciseTime::now();
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
            let pixel_t1 = PreciseTime::now();
            
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
	        	 let tracing_t1 = PreciseTime::now();
	        	 let mut color = arc_voxelizer_clone.trace(pixel_ray, max_distance, scale, max_lod);//get the color of the voxel that is hit at this direction
	        	 if iy == hcenter && jx == wcenter {
	        	    color = Color::purple();//mark the center a different color (purple)
	        	 }
	        	 let tracing_pixel_took = tracing_t1.to(PreciseTime::now());
	        	 let tracing_pixel_ns = tracing_pixel_took.num_nanoseconds().unwrap();
	        	 tx_clone.send((index,color,tracing_pixel_ns));
	        });
            /////////////////////
	        // End of Threading
	        //////////////////////
	        
	        //let color = voxelizer.trace(pixel_ray, max_distance);
            //pixels[index as usize] = color;
            cnt += 1;
            let pixel_took = pixel_t1.to(PreciseTime::now());
            let pixel_took_ns = pixel_took.num_nanoseconds().unwrap();
            total_pixel_took += pixel_took_ns;
        }
      
    }
	println!("Average pixel took: {} ms", total_pixel_took as f64/(cnt * 1_000_000) as f64);
   	let mut total_pixel_tracing_ns = 0;
    //////////////////////////////////
    // Start of Receiving thread result
    ////////////////////////////////////

	for ir in 0..screen.height{
	    for jr in range(0, screen.width){
	    	let (index,color,tracing_pixel_ns) = rx.recv().ok().expect("Could not recieve answer");
	   		pixels[index as usize] = color;
	   		total_pixel_tracing_ns += tracing_pixel_ns;
	    }
	}
	println!("Average tracing pixel took: {} ms", total_pixel_tracing_ns as f64/(cnt * 1_000_000) as f64);
	return pixels;
}


/*
//non-threaded version of rendering
fn render(voxelizer:Voxelizer, pitch:f64, yaw:f64, camera:Point, screen:Screen, scale:f64, max_distance:u64, max_lod:u8)->Vec<Color>{
    let mut pixels:Vec<Color> = Vec::new();
    let total = screen.width * screen.height;
	for t in range(0, total){
	    pixels.push(Color::new(255,255,255,255));//white background
	}
    println!("Tracing...");
    let mut cnt = 0;
    let mut percentage = 0;
	
	let pt1 = PreciseTime::now();
	let mut total_pixel_took = 0;
	let wcenter = screen.width/2;
	let hcenter = screen.height/2;
	let mut total_pixel_tracing_ns = 0;
    for iy in 0..screen.height {
        let new_percentage = ((iy as f64/screen.height as f64) * 100.0).round() as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;

        for jx in range(0,screen.width){
            let pixel_t1 = PreciseTime::now();
            
            let mut pixel_screen:Vector = screen.at_pixel(jx, iy);//direction of pixel relative to the screen facing forward
            let rotated_pixel_screen = pixel_screen.rotate_at_y(yaw);
            let rotated_at_pitch = rotated_pixel_screen.rotate_at_x(-pitch);
            let pixel_screen_camera = rotated_at_pitch.add_point(&camera);//the most correct when no rotation
            let pixel_ray = Ray::new(&camera, pixel_screen_camera);
            
            let index = iy * screen.width + jx;
	        
			//let arc_voxelizer_clone = arc_voxelizer.clone();
        	 let tracing_t1 = PreciseTime::now();
        	 let mut color = voxelizer.trace(pixel_ray, max_distance, scale, max_lod);//get the color of the voxel that is hit at this direction
        	 if iy == hcenter && jx == wcenter {
        	    color = Color::new(203, 0, 245, 255);//mark the center a different color (purple)
        	 }
        	 let tracing_pixel_took = tracing_t1.to(PreciseTime::now());
        	 let tracing_pixel_ns = tracing_pixel_took.num_nanoseconds().unwrap();
	        
            pixels[index as usize] = color;
            cnt += 1;
            let pixel_took = pixel_t1.to(PreciseTime::now());
            let pixel_took_ns = pixel_took.num_nanoseconds().unwrap();
            total_pixel_took += pixel_took_ns;
            total_pixel_tracing_ns += tracing_pixel_ns;
        }
      
    }
	println!("Average pixel took: {} ms", total_pixel_took as f64/(cnt * 1_000_000) as f64);
	println!("Average tracing pixel took: {} ms", total_pixel_tracing_ns as f64/(cnt * 1_000_000) as f64);
	return pixels;
}
*/

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

