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
use ivancerust::renderer;
use ivancerust::binvox::Binvox;


fn main(){
	render_shape()
}

fn render_shape(){
	let file_pre = "./renders/";
	let lod:u8 = 6;//lod of the object when voxelizing
	let trace_lod = 5;
	assert!(trace_lod <= lod, "trace LOD should be lesser than or equal to object LOD");
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
    
    let filename = String::from_str(format!("{}{}{}-trace{}.ppm",file_pre, shape.name(), lod, trace_lod).as_slice());

    println!("Voxelizing..{} lod: {}",shape.name(),lod);
    let mut voxelizer = Voxelizer::new(lod);
    voxelizer.start(shape);
    voxelizer.build_voxel_lod_level(trace_lod);
    println!("Done building voxel data...");
    let voxelizing_took = voxelizing_t1.to(PreciseTime::now()).num_milliseconds();
    println!("voxelizing took: {} ms",voxelizing_took);	

    let xlookat = (limit/2) as i64;
    let ylookat = (limit/2) as i64;
    let zlookat = (limit/2) as i64;

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
    let focal_distance = width/2;
    let screen = Screen::new(width, height, focal_distance, view_lod);
	let pitch = ((lookat.y - camera.y) as f64/(lookat.z - camera.z) as f64).atan();//along x
	let yaw =   ((lookat.x - camera.x) as f64/(lookat.z - camera.z) as f64).atan();//along y
	println!("pitch: {}", pitch.to_degrees());
	println!("yaw: {}", yaw.to_degrees());

    let pt1 = PreciseTime::now();

	let pixels = renderer::render(voxelizer, trace_lod, view_lod, lookat, &camera, width, height, focal_distance);
    let rendering_duration = pt1.to(PreciseTime::now());
    println!("rendering time: {} ms",rendering_duration.num_milliseconds());

    println!("filename: {}",filename);
    save_to_file(filename, pixels, width, height);
}

//save pixels to file
pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){
	let mut file = File::create(&Path::new(filename));
	let header = String::from_str("P6\n# CREATOR: lee\n");
	let size = format!("{} {}\n255\n", width, height);

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

