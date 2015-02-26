//morton.rs
pub fn encode(x:u64, y:u64, z:u64, lod:u8)->u64{
	let mut answer:u64 = 0;
	for i in range(0, lod) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}

// decode a given 64-bit morton code to an integer (x,y,z) coordinate
pub fn decode(morton:u64, lod:u8)->(u64, u64, u64){
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


pub fn a_gt_b(a:u64, b:u64, lod:u8)->bool{
	let am = decode(a, lod);
	let bm = decode(b, lod);
	am > bm
}

/*
pub fn a_gt_b(a:u64, b:u64, lod:u8)->bool{
	let limit = 1 << lod;
	let (ax, ay, az) = decode(a, lod);
	let (bx, by, bz) = decode(b, lod);
	let a_index = ax * limit * limit + ay * limit + az;
	let b_index = bx * limit * limit + by * limit + bz;
	a_index > b_index
}
*/

pub fn linear_index(morton:u64, lod:u8)->u64{
	let limit = 1 << lod;
	let (x, y, z) = decode(morton, lod);
	x * limit * limit + y * limit + z
}

