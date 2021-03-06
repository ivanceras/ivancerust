//vector.rs
use std::num::Float;
use std::fmt;
use point::Point;

pub struct Vector{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vector{

    pub fn new(x:f64, y:f64, z:f64)->Vector{
        Vector{x:x, y:y, z:z}
    }

	pub fn distance(&self)->f64{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn unit_vector(&self)->Vector{
		let d = self.distance();
		let xnew = self.x / d;
		let ynew = self.y / d;
		let znew = self.z / d;
		Vector{x:xnew, y:ynew, z:znew}
	}
	
	pub fn subtract_point(&self, point:&Point)->Vector{
		let x = self.x - point.x as f64;
		let y = self.y - point.y as f64;
		let z = self.z - point.z as f64;
		Vector::new(x,y,z)
	}
	pub fn add_point(&self, point:&Point)->Vector{
		let x = self.x + point.x as f64;
		let y = self.y + point.y as f64;
		let z = self.z + point.z as f64;
		Vector::new(x,y,z)
	}
	
	pub fn add(&self, vector:Vector)->Vector{
		let x = self.x + vector.x as f64;
		let y = self.y + vector.y as f64;
		let z = self.z + vector.z as f64;
		Vector::new(x,y,z)
	}
	
	pub fn subtract(&self, vector:Vector)->Vector{
		let x = self.x - vector.x as f64;
		let y = self.y - vector.y as f64;
		let z = self.z - vector.z as f64;
		Vector::new(x,y,z)
	}
	
	pub fn rotate(&mut self, xangle:f64, yangle:f64, zangle:f64){
	 	 self.x= yangle.cos() * self.x + yangle.sin() * xangle.sin() * self.y - yangle.sin() * xangle.cos() * self.z;
		 self.y = 0.0 + xangle.cos() * self.y + xangle.sin() * self.z;
		 self.z= yangle.sin() * self.x + yangle.cos() * - xangle.sin()  * self.y + yangle.cos() * xangle.cos() * self.z;
    }
    
    //http://stackoverflow.com/questions/13275719/rotate-a-3d-point-around-another-one
    //http://stackoverflow.com/questions/16380147/how-to-rotate-an-object-defined-by-x-y-z-points-around-the-x-y-or-z-axis
    pub fn rotate_at_z(&self, a:f64)->Vector{
    	let x = self.x * a.cos() - self.y * a.sin();
		let y = self.x * a.sin() + self.y * a.cos();
		let z = self.z;  
		Vector::new(x,y,z)
    }
     pub fn rotate_at_y(&self, b:f64)->Vector{
		let z = self.z * b.cos() - self.x * b.sin();  
		let x = self.z * b.sin() + self.x * b.cos();
		let y = self.y;
		Vector::new(x,y,z)
    }
    pub fn rotate_at_x(&self, c:f64)->Vector{
    	let y = self.y * c.cos() - self.z * c.sin();
		let z = self.y * c.sin() + self.z * c.cos();
		let x = self.x;
		Vector::new(x,y,z)
    }
    
 
	
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector { Vector{x:self.x, y:self.y, z:self.z} }
}

#[test]
fn test_rotate(){
	let mut v = Vector::new(0.0, 0.0, 1.0);
	//println!("vector: {}",v);
	v.rotate(0.0, (45.0).to_radians(), 0.0);
	//println!("rotating: {}", v);
}

#[test]
fn test_rotate_all(){
	let mut v = Vector::new(0.0, 0.0, 1.0);
	//println!("vector: {}",v);
	v.rotate((45.0).to_radians(), (45.0).to_radians(), (45.0).to_radians());
	//println!("rotating at all: {}", v);
}

#[test]
fn test_rotate_y(){
	let mut v = Vector::new(0.0, 0.0, 1.0);
	//println!("vector: {}",v);
	v.rotate_at_y((45.0).to_radians());
	//println!("rotating at y: {}", v);
}

#[test]
fn test_rotate_x(){
	let mut v = Vector::new(4.0, 8.0, 16.0);
	println!("[rotate_x] vector: {}",v);
	let x_rotated  = v.rotate_at_x((30.0).to_radians());
	println!("[rotate_x] rotating at x: {}", x_rotated);
}
//#[test]
fn test_rotate_z(){
	let mut v = Vector::new(1.0, 1.0, 1.0);
	println!("[rotate_z] vector: {}",v);
	v.rotate_at_z((45.0).to_radians());
	println!("[rotate_z] rotating at z: {}", v);	
}

#[test]
fn test_rotate_xyz(){
	let mut v = Vector::new(0.0, 0.0, 1.0);
	//println!("vector: {}",v);
	v.rotate_at_x((45.0).to_radians());
	v.rotate_at_y((45.0).to_radians());
	v.rotate_at_z((45.0).to_radians());
	//println!("rotating at xyz: {}", v);
}

