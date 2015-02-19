fn main(){
	let lod = 3;
	let limit = 1 << lod;
	let mut morton_indexes = Vec::new();
    for x in range(0,limit){
        for y in range (0, limit){
            for z in range (0, limit){
                let m = morton(x,y,z,lod);
                let (dx,dy,dz) = morton_decode(m,lod);
                morton_indexes.push(m);
                let index = x * limit * limit + y * limit + z;
                println!("index: {} = ({},{},{})   morton: {}",index,x,y,z,m);
            }
        }
    }
    
	//index: 447 = (6,7,7)   morton: 510
	//index: 448 = (7,0,0)   morton: 73
	let m1 = 510;
	let m2 = 73;
	
	println!("{} is greater than {} ?  {}",m1,m2, a_gt_b(m1,m2,lod));
	println!("index of {} is {}", m1, index_of(morton_indexes.clone(),m1, lod));
	println!("index of {} is {}", m2, index_of(morton_indexes.clone(),m2, lod));

}

pub fn morton(x:u64, y:u64, z:u64, lod:u8)->u64{
	let mut answer:u64 = 0;
	for i in range(0, lod) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}

//http://code-saturne.org/svn/saturne/branches/Version2_1/src/fvm/fvm_morton.c

pub fn a_gt_b(a:u64, b:u64, lod:u8)->bool{
	let am = morton_decode(a, lod);
	let bm = morton_decode(b, lod);
	am > bm
}

pub fn morton_decode(morton:u64, lod:u8)->(u64, u64, u64){
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;
	for i in range (0, lod) {
		x |= ((morton & ( 1  << 3 * i + 0)) >> ((3 * i) + 0)-i);
		y |= ((morton & ( 1  << 3 * i + 1)) >> ((3 * i) + 1)-i);
		z |= ((morton & ( 1  << 3 * i + 2)) >> ((3 * i) + 2)-i);
	}
	(x, y, z)
}


//uses binary search
pub fn index_of(vectors:Vec<u64>, search:u64, lod:u8)->i64{
	let mut first = 0;
	let mut last = vectors.len() - 1;
	let mut middle = (first+last)/2;
	println!("searching...");
	let mut cnt = 0;
	loop {
		if  a_gt_b(search, vectors[middle], lod) {
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
