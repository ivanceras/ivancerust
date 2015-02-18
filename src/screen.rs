//screen.rs
use vector::Vector;
use std::num::Float;
use point::Point;

pub struct Screen{
    width:i64,
    height:i64,
    fd:i64,//distance from the eye to the projection screen
    multiplier:f64,
    lod:u8
}

impl Screen{
    
    pub fn new(width:i64, height:i64,fd:i64, lod:u8)->Screen{
        let limit = 1 << lod;
        let multiplier = 2.0 * limit as f64/ width as f64; 
        println!("multiplier: {}", multiplier);
        Screen{width:width, height:height, fd:fd, multiplier:multiplier, lod:lod}
    }
    pub fn at_pixel(&self, px:i64, py:i64)->Vector{
     	let z = self.fd;
    	let x = px - self.width/2;
    	let y = self.height/2 - py;
		
        let mx = x as f64 * self.multiplier;
        let my = y as f64 * self.multiplier;
        let mz = z as f64 * self.multiplier;
        let v = Vector::new(mx, my, mz);
        v
    }
    
    //at distance 0, 1 pixel = 1 voxel
    //factor 0 lod highest detail
    //factor >0.125 to 0.25,  lod + 3   2^-3 = 0.125
    //factor >0.25 to 0.5,    lod + 2   2^-2 = 0.25
    //factor >0.5 to 1,       lod + 1   2^-1 = 0.5
    //factor >1 to 2,         lod - 0   2^0 = 1
    //factor >2 to 4,         lod - 1   2^1 = 2
    //factor >4 to 8,         lod - 2   2^2 = 4
    //factor >8 to 16,        lod - 3   2^3 = 8
    //factor >16 to 32,       lod - 4   2^4 = 16
    //
    // 2 ^ x = n
    fn get_lod_at_distance(&self, distance:f64){
        let limit = 1 << self.lod;
        let multiplier = 2.0 * limit as f64/ self.width as f64; //multiplier of width = 2 * voxel size
        //println!("multiplier {}/{} : {}",limit, self.width, multiplier);
        let fov = ((self.width as f64/2.0) / self.fd as f64).atan();
        println!("\nfov {}/{}: {}", self.width/2, self.fd, fov.to_degrees());
        let factor = distance / (multiplier * self.width as f64/2.0) * (fov/2.0).tan();//factor will be used to subtract the lod 
        let exp = factor.log2();
        println!("exponent: {}",exp);
        println!("distance {}: {} = 2 ^ {}, rounded: {}", distance, factor, exp, exp.round());//
        println!("lod: {}",self.lod as i64 - exp.round() as i64);
    }
    
}


impl Clone for Screen {
    fn clone(&self) -> Screen { Screen{width:self.width, height:self.height, fd:self.fd, multiplier:self.multiplier, lod:self.lod} }
}


#[test]
fn test_factor(){
    let width = 800;
	let height = 600;
	let fd = width*100000;// the highest the focal distance, the slow the LOD desimates
	let screen = Screen::new(width, height, fd, 5);
	screen.get_lod_at_distance(0.001);
	screen.get_lod_at_distance(0.125);
	screen.get_lod_at_distance(0.5);
	screen.get_lod_at_distance(1.0);
	screen.get_lod_at_distance(2.0);
	screen.get_lod_at_distance(4.0);
	screen.get_lod_at_distance(8.0);
	screen.get_lod_at_distance(10.0);
	screen.get_lod_at_distance(16.0);
	screen.get_lod_at_distance(32.0);
	screen.get_lod_at_distance(64.0);
	screen.get_lod_at_distance(100.0);
	screen.get_lod_at_distance(200.0);
	screen.get_lod_at_distance(400.0);
	screen.get_lod_at_distance(800.0);
	screen.get_lod_at_distance(1600.0);
	screen.get_lod_at_distance(6400.0);
	screen.get_lod_at_distance(25600.0);
}

//#[test]
fn test_screen(){
	//camera location: (10, 10, -32)
	//looking at: (16, 16, 16)
	let camera = Point{x:10, y:10, z:-32};
	let lookat = Point{x:16, y:16, z:16};
	let width = 800;
	let height = 600;
	let fd = width/2;
	let screen = Screen::new(width, height, fd, 5);
	let mut pixel_screen:Vector = screen.at_pixel(400, 300);//direction of pixel relative to the screen facing forward
	println!("\n\n");
	println!("[screen] camera: {}", camera);
	println!("[screen] looking at: {}", lookat);
	println!("[screen] pixel screen direction relative to camera: {}", pixel_screen);
	let pixel_screen_plus_camera = pixel_screen.add(camera.clone());
	println!("[screen] pixel_screen_plus_camera: {}", pixel_screen_plus_camera);
	//let yaw = -10.0;
	let yaw = ((lookat.x - camera.x ) as f64/(lookat.z - camera.z)  as f64).atan();//along y
	println!("[screen] yaw: {} ", yaw.to_degrees());
	let rotated_pixel_screen = pixel_screen.rotate_at_y(-yaw);
	println!("[screen] rotated_pixel_screen:  {}", rotated_pixel_screen);
	let rotated_pixel_screen_plus_pixel_location = rotated_pixel_screen.add_vector(pixel_screen);
	println!("[screen] rotated_pixel_screen_plus_pixel_location: {} ",rotated_pixel_screen_plus_pixel_location);
	let rotated_pixel_screen_plus_camera =  rotated_pixel_screen.add(camera.clone());
	println!("[screen] rotated_pixel_screen_plus_camera:  {}", rotated_pixel_screen_plus_camera);
	let rotated_pixel_screen_plus_pixel_location_plus_camera = rotated_pixel_screen_plus_pixel_location.add(camera.clone());
	println!("[screen] rotated_pixel_screen_plus_pixel_location_plus_camera: {}", rotated_pixel_screen_plus_pixel_location_plus_camera);
	println!("\n\n");
	
}

#[test]
fn test_screen2(){
	//camera location: (10, 10, -32)
	//looking at: (16, 16, 16)
	let camera = Point{x:10, y:10, z:-32};
	let lookat = Point{x:16, y:16, z:16};
	let width = 800;
	let height = 600;
	let fd = width/2;
	let screen = Screen::new(width, height, fd, 5);
	let mut pixel_screen:Vector = screen.at_pixel(800, 600);//direction of pixel relative to the screen facing forward
	println!("\n\n");
	println!("[screen] camera: {}", camera);
	println!("[screen] looking at: {}", lookat);
	println!("[screen] pixel screen direction relative to camera: {}", pixel_screen);
	let pixel_screen_plus_camera = pixel_screen.add(camera.clone());
	println!("[screen] pixel_screen_plus_camera: {}", pixel_screen_plus_camera);
	//let yaw = -10.0;
	let yaw = ((lookat.x - camera.x ) as f64/(lookat.z - camera.z)  as f64).atan();//along y
	println!("[screen] yaw: {} ", yaw.to_degrees());
	let rotated_pixel_screen = pixel_screen.rotate_at_y(-yaw);
	println!("[screen] rotated_pixel_screen:  {}", rotated_pixel_screen);
	let rotated_pixel_screen_plus_pixel_location = rotated_pixel_screen.add_vector(pixel_screen);
	println!("[screen] rotated_pixel_screen_plus_pixel_location: {} ",rotated_pixel_screen_plus_pixel_location);
	let rotated_pixel_screen_plus_camera =  rotated_pixel_screen.add(camera.clone());
	println!("[screen] rotated_pixel_screen_plus_camera:  {}", rotated_pixel_screen_plus_camera);
	let rotated_pixel_screen_plus_pixel_location_plus_camera = rotated_pixel_screen_plus_pixel_location.add(camera.clone());
	println!("[screen] rotated_pixel_screen_plus_pixel_location_plus_camera: {}", rotated_pixel_screen_plus_pixel_location_plus_camera);
	println!("\n\n");
	
}

//#[test]
fn test_shotcut(){
	//camera location: (10, 10, -32)
	//looking at: (16, 16, 16)
	let camera = Point{x:10, y:10, z:-32};
	let lookat = Point{x:16, y:16, z:16};
	let width = 800;
	let height = 600;
	let fd = width/2;
	let screen = Screen::new(width, height, fd, 5);
	let mut pixel_screen:Vector = screen.at_pixel(800, 600);//direction of pixel relative to the screen facing forward
	println!("\n\n");
	println!("[screen] camera: {}", camera);
	println!("[screen] looking at: {}", lookat);
	println!("[screen] pixel screen direction relative to camera: {}", pixel_screen);
	let pixel_screen_plus_camera = pixel_screen.add(camera.clone());
	println!("[screen] pixel_screen_plus_camera: {}", pixel_screen_plus_camera);
	//let yaw = -10.0;
	let yaw = ((lookat.x - camera.x ) as f64/(lookat.z - camera.z)  as f64).atan();//along y
	println!("[screen] yaw: {} ", yaw.to_degrees());
	let rotated_pixel_screen = pixel_screen.rotate_at_y(-yaw);
	let final_location = rotated_pixel_screen.add_vector(pixel_screen.clone()).add(camera.clone());
	println!("final location: {}",final_location);
	println!("\n\n");
	
}



