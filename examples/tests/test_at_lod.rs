extern crate ivancerust;

use ivancerust::voxel::Voxel;
use std::num::Float;

use ivancerust::morton;

fn main(){
	let lod1 = 7;
	let limit1 = 1 << lod1;//equivalent to 2^5
	let size1 = limit1 * limit1 * limit1;
	println!("lod1: {}, limit1:{}",lod1,limit1);
	println!("size1:{}", size1);
	
	let size = size1;
	let limit = (size as f64).cbrt();
	println!("limit: {}",limit);
	let lod = limit.log2();
	println!("lod: {}",lod);
	
}