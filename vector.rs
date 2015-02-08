//vector.rs
use std::num::Float;
use std::fmt;

pub struct Vector{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vector{

	fn distance(&self)->f64{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn unit_vector(&self)->Vector{
		let d = self.distance();
		let xnew = self.x / d;
		let ynew = self.y / d;
		let znew = self.z / d;
		Vector{x:xnew, y:ynew, z:znew}
	}
	
}

impl fmt::String for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

