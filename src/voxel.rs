//voxel.rs
use std::fmt;
use color::Color;
use morton;
use std::num::Float;

pub struct Voxel{
    lod:u8,
    limit:u64,
    //bitset:Vec<u8>,
    //indexes:Vec<u64>,
    color_indexes:Vec<u64>,
    colors:Vec<Color>,
    
}

impl Voxel{

    pub fn new(lod:u8)->Voxel{
        let mut colors = Vec::new();
        Voxel{lod:lod, limit:1<<lod, 
        /*bitset:bitset, indexes:indexes,*/ 
        colors:colors, color_indexes:Vec::new()}
     }

    
   
    //TODO need to set an unset, bit and color as well, colors should have its own storage per bits, not per 8bits
    pub fn set_bit_at_loc(&mut self, x:u64, y:u64, z:u64, val:bool, color:Color){
        let bit_index = morton::encode(x,y,z,self.lod);
	    self.set_color(bit_index,color);
	    
    }
    
    pub fn set_color(&mut self, bit_index:u64, color:Color){
		let color_index = self.index_of_color(bit_index);
		if color_index < 0 {
			self.colors.push(color);
			self.color_indexes.push(bit_index);
		}else{
			self.colors[color_index as usize] = color;
		} 
    }
    
   
    
    pub fn index_of_color(&self, search:u64)->i64{
		self.index_of(&self.color_indexes, search)
	}
	
	//http://www.programmingsimplified.com/c/source-code/c-program-binary-search
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
        	return Color::new(0,0,0,255);
        }
        self.colors[color_index as usize].clone()
    }
    
    pub fn get_color_at_loc(&self, x:i64, y:i64, z:i64)->Color{
        let m = morton::encode(x as u64, y as u64, z as u64, self.lod);
        self.get_color(m)
    }
    
    pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.color_indexes.len();
	    let new_lod = self.lod - 1;
	    let mut parent_bitset = Voxel::new(new_lod);
	    let new_limit = 1 << new_lod;
	    for i in range (0, len) {
	        let bitset_index = self.color_indexes[i];//get the bitset_index, recompute the new bit_set_index when lod = lod -1
	        let (x,y,z) = morton::decode(bitset_index, self.lod);
	        let (new_x, new_y, new_z) = self.at_lod(x as i64, y as i64, z as i64, new_lod);
			let r = 256 - ((x as f64 / new_limit as f64) * 256.0).round() as u8;
			let g = 256 - ((y as f64 / new_limit as f64) * 256.0).round() as u8;
			let b = 256 - ((z as f64 / new_limit as f64) * 256.0).round() as u8;
			let color = Color::new(r,g,b,255);
			//parent_bitset.set_bit_at_loc(x,y,z,true, color);
			parent_bitset.set_bit_at_loc(new_x as u64, new_y as u64, new_z as u64, true, color);
	    }
	    parent_bitset
    }
    
    
    
    pub fn clone(&self)->Voxel{
    	Voxel{
    	    colors:self.colors.clone(), 
    	    lod:self.lod, 
    	    limit:self.limit, 
    	    color_indexes:self.color_indexes.clone()
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
