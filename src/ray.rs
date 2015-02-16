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

	pub fn new(origin:Point, pointing:Vector, lod:u8)->Ray{
		let dir = pointing.minus(origin.clone());
		let unit_dir = dir.unit_vector();
		Ray{orig:origin, dir:dir, unit_dir: unit_dir, lod:lod}
		
	}
	
	//compute the ray at lod, with length from the origin
	pub fn at_length(&self, length:u64)->Point{
		let xlen = self.unit_dir.x * length as f64; 
		let ylen = self.unit_dir.y * length as f64;
		let zlen = self.unit_dir.z * length as f64;
		
		let xround = xlen.round() as i64;
		let yround = ylen.round() as i64;
		let zround = zlen.round() as i64;
		
		let xloc = self.orig.x + xround;
		let yloc = self.orig.y + yround;
		let zloc = self.orig.z + zround;

		Point{x:xloc, y:yloc, z:zloc}

	}
	
}
