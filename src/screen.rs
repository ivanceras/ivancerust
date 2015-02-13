//screen.rs
use vector::Vector;
use std::num::Float;

pub struct Screen{
    width:i64,
    height:i64,
    forward:Vector,//the vector of the center screen, pointing forward
    fd:i64,//distance from the eye to the projection screen
}

impl Screen{
    
    pub fn new(width:i64, height:i64, xforward:f64, yforward:f64, zforward:f64, fd:i64)->Screen{
        let forward = Vector{x:xforward, y:yforward, z:zforward};
        Screen{width:width, height:height, forward:forward.unit_vector(), fd:fd}
    }
    pub fn at_pixel(&self, px:i64, py:i64)->Vector{
     	let z = self.fd;
    	let x = px - self.width/2;
    	let y = self.height/2 - py;
    	//println!("at pixel: {},{} -> {},{}",px,py,x,y);
    	let cam_pitch = if self.forward.z == 0.0 {90.0.to_radians()} else {(self.forward.y / self.forward.z).atan()};
        let cam_yaw = if self.forward.z == 0.0 {90.0.to_radians()} else {(self.forward.x / self.forward.z).atan()};
        
        let v = Vector::new(x as f64,y as f64,z as f64);
        let uv = v.unit_vector();
        let pitch = (y as f64/z as f64).atan();
        let yaw = (x as f64/ z as f64).atan();
        let total_pitch = cam_pitch + pitch;
        let total_yaw = cam_yaw + yaw;
        //https://github.com/PistonDevelopers/cam/blob/master/src/camera.rs
        let (ys, yc, ps, pc) = (total_yaw.sin(), total_yaw.cos(), total_pitch.sin(), total_pitch.cos());
        let new_x = ys * pc;
        let new_y = ps;
        let new_z = yc * pc;
        let new_v = Vector::new(new_x, new_y, new_z);
        new_v.unit_vector()
        
    }
    
    pub fn compute_rays(&self){
        let z = self.fd;
        let cam_pitch = if self.forward.z == 0.0 {90.0.to_radians()} else {(self.forward.y / self.forward.z).atan()};
        let cam_yaw = if self.forward.z == 0.0 {90.0.to_radians()} else {(self.forward.x / self.forward.z).atan()};

        for i in range(0, self.height){
            let y = self.height/2 -i ;
            for j in range(0, self.width){
                let x = j - self.width/2;
                let v = Vector::new(x as f64,y as f64,z as f64);
                let uv = v.unit_vector();
                let pitch = (y as f64/z as f64).atan();
                let yaw = (x as f64/ z as f64).atan();
                let total_pitch = cam_pitch + pitch;
                let total_yaw = cam_yaw + yaw;
                
                let (ys, yc, ps, pc) = (total_yaw.sin(), total_yaw.cos(), total_pitch.sin(), total_pitch.cos());
                let new_x = ys * pc;
                let new_y = ps;
                let new_z = yc * pc;
                let distance = Vector::new(new_x, new_y, new_z).distance();
            }
        }
    }
    
}
