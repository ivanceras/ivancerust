//ray.rs

use voxel::Voxel;
use vector::Vector;
use std::num::Float;

pub struct Ray{
	xorig:i64,
	yorig:i64,
	zorig:i64,
	
	dir:Vector,//direction
	
	unit_dir: Vector, //unit vector of direction
	
	lod:u8,
}

impl Ray{

	pub fn new(xorig:i64, yorig:i64, zorig:i64, dir:Vector, lod:u8)->Ray{
		let limit:u64 = 1 << lod;
		let xlimit = limit;
        let ylimit = limit;
        let zlimit = limit;
		println!("calc_total {}", (zlimit * ylimit * zlimit));
		let unit_dir = dir.unit_vector();
		println!("unit vector: {}",unit_dir);
		Ray{xorig:xorig, yorig:yorig, zorig:zorig, dir:dir, unit_dir: unit_dir, lod:lod}
		
	}
	
	//compute the ray at lod, with length from the origin
	pub fn at_length(&self, length:u64)->(i64,i64,i64){
		let xlen = self.unit_dir.x * length as f64; 
		let ylen = self.unit_dir.y * length as f64;
		let zlen = self.unit_dir.z * length as f64;
		
		//println!("len: {}, {}, {}",xlen, ylen, zlen);
		let xround = xlen.round() as i64;
		let yround = ylen.round() as i64;
		let zround = zlen.round() as i64;
		//println!("rounded: {}, {}, {}",xround, yround, zround);
		
		let xloc = self.xorig + xround;
		let yloc = self.yorig + yround;
		let zloc = self.zorig + zround;

		//println!("loc: {}, {}, {}",xloc, yloc, zloc);
		(xloc, yloc, zloc)

	}
	
}

fn morton(x:u64, y:u64, z:u64, lod:u8)->u64{
	let mut answer:u64 = 0;
	
	for i in range(0, lod) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}
