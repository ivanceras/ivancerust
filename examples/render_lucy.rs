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
	render_lucy();
}


fn render_lucy(){
	let voxel = Binvox::read_file("data/lucy8.binvox");
	let lod:u8 = voxel.lod;//lod of the object when voxelizing
	let trace_lod = 8;
    let limit:u64 = 1 << lod;
    let scale = 1.0;

	let view_lod = 8;//base LOD at screen 1 voxel = 1 pixel
    let view_limit = 1 << view_lod;

    
 
    //look at the center of the sphere
    let xlookat = (limit/2) as i64;
    let ylookat = (limit/2) as i64;
    let zlookat = (limit/2) as i64;

    let xcam = (limit/2) as i64;
    let ycam = (limit/2) as i64;
    let zcam = -100;
    
    let camera = Point{x:xcam, y:ycam, z:zcam};
    let lookat = Point{x:xlookat, y:ylookat, z:zlookat};
    
    let max_distance = 4 * view_limit as u64 + zcam.abs() as u64;
	
	
    let width = 800;
    let height = 800;
    let focal_distance = width/2;
    
    let mut voxelizer = Voxelizer::new(lod);
    voxelizer.set_voxel(voxel);
    voxelizer.build_voxel_lod_level(trace_lod);
    
	let pixels = renderer::render(voxelizer, trace_lod, view_lod, lookat, &camera, width, height, focal_distance);
    let filename = format!("renders/lucy{}-trace{}.ppm",lod,trace_lod);
    println!("file: {}",filename);
    save_to_file(filename, pixels, width, height);
}


//save pixels to file
pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){
	let mut file = File::create(&Path::new(filename));
	let header = format!("P6\n# CREATOR: lee\n");
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

