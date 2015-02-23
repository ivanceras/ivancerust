//shape.rs
use point::Point;
use std::num::Float;
use vector::Vector;
use normal::Normal;


pub trait Shape{
    fn is_inside(&self, point:&Point)->bool;
    fn normal(&self, point:&Point)->Normal;
    fn name(&self)->String;
}


pub struct Sphere{
    radius:u64,
    center:Point
}

pub struct Cube{
    lower:Point,
    upper:Point
}

struct Prism{
    bound1:Point,//is inside when x >= bound1.x && x <= bound2.x
    bound2:Point,//               y >= bound1.y && y <= bound2.y
                 //               z >= bound1.z && z <= bound2.z     
}

struct Cylinder{
    radius:u64,
    height:u64,
}



impl Sphere{
    pub fn new(radius:u64, center:&Point)->Sphere{
        Sphere{radius:radius, center:center.clone()}
    }
    
    
}

impl Shape for Sphere{

    fn is_inside(&self, point:&Point)->bool{
        let xf = (point.x - self.center.x) as f64;
		let yf = (point.y - self.center.y) as f64;
		let zf = (point.z - self.center.z) as f64;
		let rad = (xf*xf + yf*yf + zf*zf).sqrt().round() as u64;
		if rad <= self.radius {
			return true;
		}
		false
    }
    
    fn normal(&self, point:&Point)->Normal{
        let p = Vector::new(point.x as f64, point.y as f64, point.z as f64);
        let cp = p.minus(&self.center);
        let normal = Normal::from_vector(&cp);
        normal
    }
    
    fn name(&self)->String{
        "sphere".to_string()
    }
}

impl Cube{
    pub fn new(radius:u64, center:&Point)->Cube{
        
        let xlowerbound = center.x - radius as i64;
        let ylowerbound = center.y - radius as i64;
        let zlowerbound = center.z - radius as i64;
        
        let xupperbound = center.x + radius as i64;
        let yupperbound = center.y + radius as i64;
        let zupperbound = center.z + radius as i64;
        
        let lowerbound = Point::new(xlowerbound, ylowerbound, zlowerbound);
        let upperbound = Point::new(xupperbound, yupperbound, zupperbound);
        
        Cube{lower:lowerbound, upper:upperbound}
    }
}



impl Shape for Cube{

    fn is_inside(&self, point:&Point)->bool{

		if point.x >= self.lower.x && point.x <= self.upper.x &&
           point.y >= self.lower.y && point.y <= self.upper.y &&
		   point.z >= self.lower.z && point.z <= self.upper.z {
			return true;
		}
		false
    }
    fn normal(&self, point:&Point)->Normal{
        let p = Vector::new(0.0, 0.0, 0.0);
        let normal = Normal::from_vector(&p);
        normal
    }
    fn name(&self)->String{
        "cube".to_string()
    }
}
