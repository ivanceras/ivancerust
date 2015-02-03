//voxel of a sphere
//32 bit LOD, r^2 = x^2 + y^2 + z^2
//This calculates the approximate volume of a sphere

use std::num::Float;
fn main(){
	let bits:u8 = 11;
    let limit:u64 = 1 << bits;
    let r:u64 = 1 << bits-1;

    println!("bits: {}", bits);
    println!("limit: {}", limit);
    println!("radius: {}", r);
    let max_morton = morton(limit-1, limit-1, limit-1, bits);
    println!("max_morton: {}", max_morton);
    bit_index(max_morton);
    //set the center
    
    let xlimit = limit;
    let ylimit = limit;
    let zlimit = limit;
    
    make_array(xlimit, ylimit, zlimit);
    
    let cx = xlimit/2;
    let cy = ylimit/2;
    let cz = zlimit/2;
    
    let mut inside:u64 = 0;
    let mut outside:u64 = 0;
    let calc_total:f64 = (xlimit as f64) * (ylimit as f64) * (zlimit as f64);
     println!("calc_total {} or {}", calc_total, (xlimit * ylimit * zlimit));
    let mut actual_total:u64 = 0;
    let mut percentage:u64 = 0;
    let mut inline:u64 = 0;
    let mut counter:u64 = 0;
    
    for i in range (0, xlimit){
        let new_percentage = (i as f64/limit as f64 * 100.0) as u64;
        if new_percentage != percentage {
            println!("{} %", percentage);
        }
        percentage = new_percentage;
        for j in range (0, ylimit) {
            for k in range (0, zlimit){
                  actual_total += 1;
                  let x = (i - cx) as f64;
                  let y = (j - cy) as f64;
                  let z = (k - cz) as f64;
                  let iijjkk:f64 = x*x + y*y + z*z ;
                  let rr:f64 = (r as  f64) * (r as f64);
                  let sqrt_iijjkk:f64 = (iijjkk).sqrt();
                  let rounded_sqrt_ijk = (sqrt_iijjkk + 0.5 ) as u64;
                  
                  //http://nadeausoftware.com/articles/2012/06/c_c_tip_how_loop_through_multi_dimensional_arrays_quickly#Method1Nestedloopswithmultiplearrays
                  //i * height * depth + j * depth + k ]
                  let index = i * ylimit * zlimit + j * zlimit + k;
                  let m = morton(i,j,k, bits);
                  //println!("morton: {}",m);
                  assert!(index == counter);
                  
                  if rounded_sqrt_ijk == r {
                    inside += 1;
                    inline += 1;
                  }
                  
                  else if sqrt_iijjkk < r as f64 {
                    inside += 1;
                  }
                  else {
                    outside += 1;
                  }
                  counter += 1;
              }
        }    
    }
    
    println!("inside: {}", inside);
    println!("outside: {}", outside);
    println!("inline: {}", inline);
    println!("calc_total: {}", calc_total);
    println!("actual_total: {} or {}", actual_total, inside+outside);
    let rf64 = r as f64;
    println!("calculated volume = {} ", (4.0/3.0 * std::f64::consts::PI * rf64 * rf64 * rf64));
    println!("calculated surface area = {} ", (4.0 * std::f64::consts::PI * rf64 * rf64));
    println!("percentage of inside {}", inside as f64/ actual_total as f64);
    
}


fn morton(x:u64, y:u64, z:u64, bits:u8)->u64{
	let mut answer:u64 = 0;
	
	for i in range(0, bits) {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}

fn make_array(xlimit:u64, ylimit:u64, zlimit:u64){
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
}

fn bit_index(index:u64){
	let byte_index = index / 8;
	let remainder = index % 8;
	println!("byte:[{}], rem: {}",byte_index, remainder);
}
