//voxel of a sphere
//32 bit LOD, r^2 = x^2 + y^2 + z^2
//This calculates the approximate volume of a sphere

use std::num::Float;
fn main(){
	let lod:u8 = 11;
    let limit:u64 = 1 << lod;
    let r:u64 = 1 << lod-2;//do a radius of half the limit

    println!("lod: {}", lod);
    println!("limit: {}", limit);
    println!("radius: {}", r);
    //set the center
    
    let xlimit = limit;
    let ylimit = limit;
    let zlimit = limit;
    
    
    
    let cx = xlimit/2;
    let cy = ylimit/2;
    let cz = zlimit/2;
    
    println!("center: {},{},{}",cx,cy,cz);
    
    let mut inside:u64 = 0;
    let mut outside:u64 = 0;
    let calc_total:f64 = (xlimit as f64) * (ylimit as f64) * (zlimit as f64);
    println!("calc_total {} or {}", calc_total, (xlimit * ylimit * zlimit));
    let mut actual_total:u64 = 0;
    let mut percentage:u64 = 0;
    let mut inline:u64 = 0;
    let mut counter:u64 = 0;
    
    let max_morton = morton(limit-1, limit-1, limit-1, lod);
    println!("max_morton: {}", max_morton);
    let mut bitset = make_array(xlimit, ylimit, zlimit);
    let mut grid = make_array(xlimit, ylimit, zlimit);
    //set_bit(&mut bitset, max_morton, true);

    
    for i in range (0, xlimit){
        let new_percentage = (i as f64/limit as f64 * 100.0) as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;
        for j in range (0, ylimit) {
            for k in range (0, zlimit){
                  actual_total += 1;
                  //sign matters here
                  let x = (i as i64 - cx as i64) as f64;
                  let y = (j as i64 - cy as i64) as f64;
                  let z = (k as i64 - cz as i64) as f64;
                  //println!("ijk:    {}, {}, {}",i,j,k);
                  //println!("cxcycz: {}, {}, {}",cx,cy,cz);
                  //println!("xyz:    {}, {}, {}",x,y,z);
                  
                  //println!("(i-cx),(j-cy),(k-cz): {}, {}, {}", (i - cx),(j - cy),(k - cz));
                  let iijjkk:f64 = x*x + y*y + z*z ;
                  let sqrt_iijjkk:f64 = (iijjkk).sqrt();
                  let rounded_sqrt_ijk = (sqrt_iijjkk.round()) as u64;
                  
                  //http://nadeausoftware.com/articles/2012/06/c_c_tip_how_loop_through_multi_dimensional_arrays_quickly#Method1Nestedloopswithmultiplearrays
                  //i * height * depth + j * depth + k ]
                  let index = i * ylimit * zlimit + j * zlimit + k;
                  let m = morton(i,j,k, lod);
                  //println!("morton: {}",m);
                  assert!(index == counter);
                  
                  if rounded_sqrt_ijk == r {
                    inside += 1;
                    inline += 1;
                    set_bit(&mut bitset, m, true);
                    set_bit(&mut grid, index, true);
                  }
                  
                  else if sqrt_iijjkk < r as f64 {
                    inside += 1;
                    set_bit(&mut bitset, m, true);
                    set_bit(&mut grid, index, true);
                  }
                  else {
                    outside += 1;
                  }
                  counter += 1;
              }
        }    
    }
    let ret_bitset = display_bitset(bitset);
    println!("inside: {}", inside);
    println!("outside: {}", outside);
    println!("inline: {}", inline);
    println!("calc_total: {}", calc_total);
    println!("actual_total: {} or {}", actual_total, inside+outside);
    let rf64 = r as f64;
    println!("calculated volume = {} ", (4.0/3.0 * std::f64::consts::PI * rf64 * rf64 * rf64));
    println!("calculated surface area = {} ", (4.0 * std::f64::consts::PI * rf64 * rf64));
    println!("percentage of inside {}", inside as f64/ actual_total as f64);
    
    //let parent = parent_bitset(ret_bitset);
    //let ret_parent = display_bitset(parent);
    //let grand_parent = parent_bitset(ret_parent);
    //let ret_grand_parent = display_bitset(grand_parent);	
    //let great_grand_parent = parent_bitset(ret_grand_parent);//ideal for 5 LOD
    //display_bitset(great_grand_parent);

    println!("GRID..");
    display_bitset(grid);
    
    let mut parent = ret_bitset;
   	for l in range (0, lod-1){
    	parent = parent_bitset(parent);
    	println!("ITERATION: {}",l);
    	display_bitset_slice(parent.as_slice());
    }
}


fn morton(x:u64, y:u64, z:u64, lod:u8)->u64{
	let mut answer:u64 = 0;
	
	for i in range(0, lod) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}


//make an array of bitsets which can hold voxels of size xlimit, ylimit and zlimit
fn make_array(xlimit:u64, ylimit:u64, zlimit:u64)->Vec<u8>{
	let mut bitset:Vec<u8> = Vec::new();
	for i in range (0, xlimit){
        for j in range (0, ylimit) {
            for k in range (0, zlimit){
            	let index = i * ylimit * zlimit + j * zlimit + k;
            	let byte_index = index / 8;
            	if index % 8 == 0 {
            		bitset.push(0);
            	}
            }
        }
    }
    println!("bitset length: {}",bitset.len());
    bitset
}

//make an array of bitsets which can hold a voxel of size len
fn make_array_of_len(voxel_capacity:u64)->Vec<u8>{
	let mut bitset:Vec<u8> = Vec::new();
	for i in range(0, voxel_capacity){
		if i % 8 == 0 {
			bitset.push(0);
		}
	}
	bitset
}

fn set_bit(bitset:&mut Vec<u8>, index:u64, val:bool){

	let byte_index = (index / 8) as usize;
	let remainder = index % 8;
	//println!("byte_index:[{}], rem: {}",byte_index, remainder);
	let byte = bitset[byte_index];
	//println!("byte_value: {} ", byte);
	//println!("new value at {}: {}", remainder, val);
	if val {
	    let or_byte = 1 << remainder;
	    let new_byte = byte | or_byte;
        bitset[byte_index] = new_byte;
        //println!("new_byte: {}", new_byte);
	}
}

//get the parent bit_set of this bitset
fn parent_bitset(bitset: Vec<u8>)->Vec<u8>{
	println!("Getting the parent bitset...");
	let len = bitset.len();
	let voxel_capacity:u64 = len as u64 * 8;
	println!("voxel_capacity: {}",voxel_capacity);
	println!("Grouped by 8, it will then have {}",voxel_capacity/8);
	let parent_voxel_size = voxel_capacity as u64 / 8; //which is obviously still equal to len, but it is easier to think in voxel context
	let mut parent_bitset = make_array_of_len(parent_voxel_size);
	for i in range (0, len) {
		let byte = bitset[i];
		let index = i as u64;//index of the bit of the parent, not the index of the byte
		if byte > 0 {
			set_bit(&mut parent_bitset, index, true);
		}
	}
	parent_bitset
}

fn display_bitset(bitset: Vec<u8>)->Vec<u8>{
    let len = bitset.len();
    for i in range (0, len) {
        println!("[{}]: {}",i, bitset[i]);
    }
    bitset
}

fn display_bitset_slice(bitset: &[u8]){
    let len = bitset.len();
    for i in range (0, len) {
        println!("[{}]: {}",i, bitset[i]);
    }
}
