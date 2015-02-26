//voxel.rs
use std::fmt;
use color::Color;
use morton;
use std::num::Float;

pub struct Voxel{
    pub lod:u8,
    limit:u64,
    pub indexes:Vec<u64>,
    colors:Vec<Color>,
    
}

impl Voxel{

    pub fn new(lod:u8)->Voxel{
        Voxel{
		    lod:lod, 
		    limit:1<<lod, 
		    colors:Vec::new(), 
		    indexes:Vec::new()
		}
     }

    
   
    //TODO need to set an unset, bit and color as well, colors should have its own storage per bits, not per 8bits
    pub fn set_bit_at_loc(&mut self, x:u64, y:u64, z:u64, val:bool, color:Color){
        let bit_index = morton::encode(x,y,z,self.lod);
	    self.set_color(bit_index,color);
	    
    }
    
    pub fn set_color(&mut self, bit_index:u64, color:Color){
		let color_index = self.index_of_color(bit_index);
		//if self.lod == 1 {
		//	println!("color_index of {} : {} lod:{} ",bit_index, color_index, self.lod);
		//}
		if color_index < 0 {
			//println!("at LOD:{} bit_index {} color_index: {} putting color:{} ",self.lod ,bit_index, color_index, color);
			self.colors.push(color);
			self.indexes.push(bit_index);
			//for i in range(0, self.indexes.len()){
			//	println!("indexes[{}]: {}",i, self.indexes[i]);
			//}
		}else{
			//if self.lod == 1 {
			//	println!("at LOD:{} setting color_index: {} with color: {}",self.lod, color_index, color);
			//}
			self.colors[color_index as usize] = color;
		} 
    }
    
   
	//http://www.programmingsimplified.com/c/source-code/c-program-binary-search
	/*
	pub fn index_of(&self, vector:&Vec<u64>, search:u64)->i64{
		let mut first = 0 as i64;
		let mut last = vector.len() as i64 - 1;
		let mut middle = (first+last)/2;
		while first <= last {
			if  morton::a_gt_b(search, vector[middle as usize], self.lod) {
				first = middle + 1;    
			}
			else if  vector[middle as usize] == search  {
			 	return middle as i64;
			}
			else {
			 	last = middle - 1;
			}
			middle = (first + last)/2;
		}
		-1
	}
    */
    
    fn iterative_index_of(&self, vector:&Vec<u64>, search:u64)->i64{
    	for i in range(0, vector.len()){
    		if vector[i] == search {
    			return i as i64;
    		}
    	}
    	-1
    }
    
    pub fn index_of_color(&self, search:u64)->i64{
		let mut first = 0 as i64;
		let mut last = self.indexes.len() as i64 - 1;
		let mut middle = (first+last)/2;
		while first <= last {
			if  morton::a_gt_b(search, self.indexes[middle as usize], self.lod) {
				first = middle + 1;    
			}
			else if  self.indexes[middle as usize] == search  {
			 	return middle as i64;
			}
			else {
			 	last = middle - 1;
			}
			middle = (first + last)/2;
		}
		-1
	}
    
    pub fn isset(&self, x:i64, y:i64, z:i64)->bool{
    	let bit_index = morton::encode(x as u64, y as u64, z as u64, self.lod);
		let color_index = self.index_of_color(bit_index);
		if color_index < 0 {
			return false
		}
		true
	}
    
    pub fn get_color(&self, bit_index:u64)->Color{
        let color_index = self.index_of_color(bit_index);
        if color_index < 0 {
        	return Color::white();
        }
        self.colors[color_index as usize].clone()
    }
    
    pub fn get_color_at_loc(&self, x:i64, y:i64, z:i64)->Color{
        let m = morton::encode(x as u64, y as u64, z as u64, self.lod);
        self.get_color(m)
    }
    
    pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let new_lod = self.lod - 1;
	    //create a sorted array of linear index
	    let mut linear_indexes = Vec::new();
	    let mut linear_color_indexes = Vec::new();
	    let mut linear_morton = Vec::new();
	    let new_limit = 1 << new_lod;
	    for i in range (0, len) {
	        let bitset_index = self.indexes[i];//get the bitset_index, recompute the new bit_set_index when lod = lod -1
	        let (x,y,z) = morton::decode(bitset_index, self.lod);
	        let (new_x, new_y, new_z) = self.at_lod(x as i64, y as i64, z as i64, new_lod);
			let r = 256 - ((x as f64 / new_limit as f64) * 255.0).round() as u8;
			let g = 256 - ((y as f64 / new_limit as f64) * 255.0).round() as u8;
			let b = 256 - ((z as f64 / new_limit as f64) * 255.0).round() as u8;
			let color = Color::new(r/2,g/2,b/2,255);
			let new_morton = morton::encode(new_x as u64, new_y as u64, new_z as u64, new_lod);
			let linear_index = morton::linear_index(new_morton, new_lod);
			let original_index = morton::linear_index(bitset_index, self.lod);
			let iter_index = self.iterative_index_of(&linear_indexes, linear_index);
			if iter_index < 0 {
				linear_indexes.push(linear_index);
				linear_color_indexes.push(color.clone());
				linear_morton.push(new_morton);
			}
			else{
				linear_indexes[iter_index as usize] = linear_index;
				
				let prev_color = linear_color_indexes[iter_index as usize].clone();
				let r_ave = (prev_color.r + color.r/2);
				let g_ave = (prev_color.g + color.g/2);
				let b_ave = (prev_color.b + color.b/2);
				let a_ave = 255;
				let average_color = Color::new(r_ave as u8,g_ave as u8,b_ave as u8,a_ave as u8);
				println!("average color:{}", average_color);
				linear_color_indexes[iter_index as usize] = average_color;
				linear_morton[iter_index as usize] = new_morton;
			}
	    }
	    
	    let original_indexes = linear_indexes.clone(); 
	    linear_indexes.sort();
  	    let mut parent_bitset = Voxel::new(new_lod);
		
	    for s in range(0, linear_indexes.len()){
	     	let linear_index = linear_indexes[s];
	     	let orig_index = self.iterative_index_of(&original_indexes, linear_index);
	    	let color = linear_color_indexes[orig_index as usize].clone();
	    	let morton = linear_morton[orig_index as usize];
	    	parent_bitset.set_color(morton, color)
	    }
	    parent_bitset
    }
    
    
    
    pub fn clone(&self)->Voxel{
    	Voxel{
    	    colors:self.colors.clone(), 
    	    lod:self.lod, 
    	    limit:self.limit, 
    	    indexes:self.indexes.clone()
    	}
    }
    
     //get the x,y,z at given lod
    pub fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	//println!("at lod {}: {},{},{}", new_lod, xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    //determines if the point is inside the boundary of this voxel
     pub fn bounded(&self, x:i64, y:i64, z:i64)->bool{
    	let xlowerbound = 0;
    	let ylowerbound = 0;
    	let zlowerbound = 0;
 		let xupperbound = self.limit as i64;
 		let yupperbound = self.limit as i64;
 		let zupperbound = self.limit as i64;
 		if x < xlowerbound || y < ylowerbound || z < zlowerbound 
 		|| x > xupperbound || y > yupperbound || z > zupperbound
 		{
 			return false;
 		}
 		true
    }
    
    //determine if it hits a voxel, this is direct test wihout using the lower level LOD
    pub fn hit(&self, x:i64, y:i64, z:i64)->bool{
    	let bounded = self.bounded(x, y, z);
    	if !bounded {
    		return false;
    	}
    	let isset = self.isset(x,y,z);
    	isset
    }
    
    
}

fn count_bits(arg:u8)->u8 {
    let mut count:u8 = 0;
    let mut x = arg;
    while x > 0 {
        x &= x-1;
        count += 1;
    }
    count
}


impl fmt::Display for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let len = self.indexes.len();
    	let mut buffer = String::new();
    	for i in range(0, len){
    		let index = self.indexes[i];
        	buffer.push_str(format!("index[{}] = {} color[{}] = {}\n",i, self.indexes[i], i, self.colors[i]).as_slice());
        }
        write!(f, "{}", buffer)
    }
}
