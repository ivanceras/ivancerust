//ray.rs

use voxel::Voxel;
use vector::Vector;
use std::num::Float;
use point::Point;

pub struct Ray{
	
	orig:Point,
	dir:Vector,//direction
	unit_dir: Vector, //unit vector of direction
	lod:u8,
}

impl Ray{

	pub fn new(xorig:i64, yorig:i64, zorig:i64, dir:Vector, lod:u8)->Ray{
		let orig = Point{x:xorig, y:yorig, z:zorig};
		let limit:u64 = 1 << lod;
		let xlimit = limit;
        let ylimit = limit;
        let zlimit = limit;
		//println!("calc_total {}", (zlimit * ylimit * zlimit));
		let unit_dir = dir.unit_vector();
		//println!("unit vector: {}",unit_dir);
		Ray{orig:orig, dir:dir, unit_dir: unit_dir, lod:lod}
		
	}
	
	//compute the ray at lod, with length from the origin
	pub fn at_length(&self, length:u64)->Point{
		let xlen = self.unit_dir.x * length as f64; 
		let ylen = self.unit_dir.y * length as f64;
		let zlen = self.unit_dir.z * length as f64;
		
		//println!("len: {}, {}, {}",xlen, ylen, zlen);
		let xround = xlen.round() as i64;
		let yround = ylen.round() as i64;
		let zround = zlen.round() as i64;
		//println!("rounded: {}, {}, {}",xround, yround, zround);
		
		let xloc = self.orig.x + xround;
		let yloc = self.orig.y + yround;
		let zloc = self.orig.z + zround;

		//println!("loc: {}, {}, {}",xloc, yloc, zloc);
		Point{x:xloc, y:yloc, z:zloc}

	}
	
}

