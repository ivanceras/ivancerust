extern crate ivancerust;

use ivancerust::morton::Morton;
use std::num::Float;

fn main(){
	let lod = 5;
	let limit = 1 << lod;
	let mut morton_indexes = Vec::new();
	let morton = Morton::new(lod);
    let new_lod = lod - 1;
	let new_limit = 1 << lod;
    let div = 1 << (lod - new_lod);

	
    for x in range(0,limit){
        for y in range (0, limit){
            for z in range (0, limit){
                let m = morton.encode(x,y,z);
                let (dx,dy,dz) = morton.decode(m);
                morton_indexes.push(m);
                let index = x * limit * limit + y * limit + z;
                assert!((x,y,z) == (dx, dy, dz));
                let linear_index = morton.linear_index(m);
                let (fx, fy, fz) = morton.from_linear_index(linear_index);
                assert!((x,y,z) == (fx, fy, fz));
                println!("xyz:{},{},{} from_linear_index:{},{},{}",x,y,z,fx,fy,fz);
                assert!(index == linear_index);
                println!("linear index = {}", linear_index);
                println!("index: {} = ({},{},{})   morton: {}",index,x,y,z,m);
                let (nx,ny,nz) = morton.point_at_lod(x,y,z,new_lod);
                println!("point at lod:{} ({},{},{})",new_lod,nx,ny,nz);
                let xnew = (x as f64 / div as f64).round() as i64;
		    	let ynew = (y as f64 / div as f64).round() as i64;
		    	let znew = (z as f64 / div as f64).round() as i64;
		    	println!("calculated: {},{},{}", xnew, ynew, znew);
                let xnr = x  / div;
		    	let ynr = y  / div;
		    	let znr = z  / div;
		    	println!("no rounding: {},{},{}", xnr, ynr, znr);
            }
        }
    }
    
	//index: 447 = (6,7,7)   morton: 510
	//index: 448 = (7,0,0)   morton: 73
	let m1 = 510;
	let m2 = 73;
	assert!(morton.a_gt_b(m2,m1));
	println!("{} is greater than {} ?  {}",m1,m2, morton.a_gt_b(m1,m2));
	println!("index of {} is {}", m1, index_of(morton_indexes.clone(),m1, lod));
	println!("index of {} is {}", m2, index_of(morton_indexes.clone(),m2, lod));

}

pub fn index_of(vectors:Vec<u64>, search:u64, lod:u8)->i64{
	let mut first = 0;
	let mut last = vectors.len() - 1;
	let mut middle = (first+last)/2;
	println!("searching...");
	let mut cnt = 0;
	let morton = Morton::new(lod);
	loop {
		if  morton.a_gt_b(search, vectors[middle]) {
			first = middle + 1;    
		}
		else if  vectors[middle] == search  {
		 	return middle as i64;
		}
		else {
		 	last = middle - 1;
		}
		middle = (first + last)/2;
		if first > last {
			return -1;
		}
		println!("looped: {}", cnt);
		cnt += 1;
	}
	
}
