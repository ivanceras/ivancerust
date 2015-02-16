//point.rs

use std::fmt;

pub struct Point{
	pub x:i64,
	pub y:i64,
	pub z:i64
}

impl fmt::String for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point { Point{x:self.x, y:self.y, z:self.z} }
}
