//ray.rs

use vector::Vector;
use std::num::Float;
use point::Point;

pub struct Ray{
	
	orig:Point,
	dir:Vector,//direction
	unit_dir: Vector, //unit vector of direction
}

impl Ray{

	pub fn new(origin:&Point, pointing:Vector)->Ray{
		let dir = pointing.subtract_point(origin);
		let unit_dir = dir.unit_vector();
		Ray{orig:origin.clone(), dir:dir, unit_dir: unit_dir}
		
	}
	
	pub fn at_length(&self, length:u64)->Vector{
		let xlen = self.orig.x as f64 + self.unit_dir.x * length as f64; 
		let ylen = self.orig.y as f64 + self.unit_dir.y * length as f64;
		let zlen = self.orig.z as f64 + self.unit_dir.z * length as f64;
		Vector::new(xlen, ylen, zlen)

	}
	
}

