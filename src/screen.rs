//screen.rs
use vector::Vector;
use std::num::Float;
use point::Point;

pub struct Screen{
    width:i64,
    height:i64,
    fd:i64,//distance from the eye to the projection screen
    multiplier:f64,
}

impl Screen{
    
    pub fn new(width:i64, height:i64,fd:i64, lod:u8)->Screen{
        let limit = 1 << lod;
        let multiplier = limit as f64/ width as f64;
        //let multiplier = 1.0;
        println!("multiplier: {}", multiplier);
        Screen{width:width, height:height, fd:fd, multiplier:multiplier}
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



