extern crate ivancerust;

use ivancerust::morton;

fn main(){
	let lod = 3;
	let limit = 1 << lod;
	let mut morton_indexes = Vec::new();
    for x in range(0,limit){
        for y in range (0, limit){
            for z in range (0, limit){
                let m = morton::encode(x,y,z,lod);
                let (dx,dy,dz) = morton::decode(m,lod);
                morton_indexes.push(m);
                let index = x * limit * limit + y * limit + z;
                assert!((x,y,z) == (dx, dy, dz));
                println!("index: {} = ({},{},{})   morton: {}",index,x,y,z,m);
            }
        }
    }
    
	//index: 447 = (6,7,7)   morton: 510
	//index: 448 = (7,0,0)   morton: 73
	let m1 = 510;
	let m2 = 73;
	assert!(morton::a_gt_b(m2,m1,lod));
	println!("{} is greater than {} ?  {}",m1,m2, morton::a_gt_b(m1,m2,lod));
	println!("index of {} is {}", m1, index_of(morton_indexes.clone(),m1, lod));
	println!("index of {} is {}", m2, index_of(morton_indexes.clone(),m2, lod));

}

pub fn index_of(vectors:Vec<u64>, search:u64, lod:u8)->i64{
	let mut first = 0;
	let mut last = vectors.len() - 1;
	let mut middle = (first+last)/2;
	println!("searching...");
	let mut cnt = 0;
	loop {
		if  morton::a_gt_b(search, vectors[middle], lod) {
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
