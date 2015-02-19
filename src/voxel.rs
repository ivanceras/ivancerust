//voxel.rs
use std::fmt;
use color::Color;
use morton;
use std::num::Float;
use std::collections::Bitv;

pub struct Voxel{
    lod:u8,
    limit:u64,
    bitset:Vec<u8>,
    indexes:Vec<u64>,
    color_indexes:Vec<u64>,
    colors:Vec<Color>,
    
}

impl Voxel{

    pub fn new(lod:u8)->Voxel{
        let mut bitset = Vec::new();
        let mut indexes = Vec::new();
        let mut colors = Vec::new();
        Voxel{lod:lod, limit:1<<lod, bitset:bitset, indexes:indexes, colors:colors, color_indexes:Vec::new()}
     }

    
   
    
    fn set_bit(&mut self, bit_index:u64, val:bool){
	    let byte_index = (bit_index / 8) as u64;
        let remainder = bit_index % 8;
        let mut bitset_index = self.index_of_bitset(byte_index);
        if bitset_index < 0 {
            self.indexes.push(byte_index);
            self.bitset.push(0);
            bitset_index = self.index_of_bitset(byte_index);
        }
        let byte = self.bitset[bitset_index as usize];
        if val {
            let or_byte = 1 << remainder;
            let new_byte = byte | or_byte;
            self.bitset[bitset_index as usize] = new_byte;
        }
        else{
        	let tmp_byte:u8 = 1 << remainder;
        	let and_byte:u8 = !tmp_byte;//flip the bits
        	let new_byte = byte & and_byte;
        	self.bitset[bitset_index as usize] = new_byte;
        }
	    
    }
    
    
    //TODO need to set an unset, bit and color as well, colors should have its own storage per bits, not per 8bits
    pub fn set_bit_at_loc(&mut self, x:u64, y:u64, z:u64, val:bool, color:Color){
        let bit_index = morton::encode(x,y,z,self.lod);
	    self.set_bit(bit_index, val);
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
		let mut cnt = 0;
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
			cnt += 1;
		}
		-1
	}
    
    //since bitset indexes are divided by 8, the comparing of the bitsets does not apply anymore
    pub fn index_of_bitset(&self, value:u64)->i64{
        for i in range(0, self.indexes.len()){
            if(value == self.indexes[i]){
                return i as i64;
            }
        }
        -1
    }
    
    
    
    //check to see if bit is set or not
    pub fn isset(&self, bit_index:u64)->bool{
    	let byte_index = (bit_index / 8) as u64;
        let remainder = bit_index % 8;
        let mut bitset_index = self.index_of_bitset(byte_index);
        if bitset_index < 0 {
        	return false;
        }
        let byte = self.bitset[bitset_index as usize];
        let and_byte = 1 << remainder;
        if byte & and_byte > 0 {
        	return true;
        }
    	false
    }
    
    
    
    pub fn get_color(&self, bit_index:u64)->Color{
        let color_index = self.index_of_color(bit_index);
        if color_index < 0 {
        	return Color{r:0,g:0,b:0};
        }
        self.colors[color_index as usize].clone()
    }
    
    pub fn show_indexes(&self){
        println!("There are {} occupied", self.indexes.len());
        for i in self.indexes.iter(){
            println!("{}",i);
        }
    }
    
    //get the parent bit_set of this bitset
    //the bitset index should be converted to morton first before doing the operation
     pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let new_lod = self.lod - 1;
	    let mut parent_bitset = Voxel::new(new_lod);
	    for i in range (0, len) {
	        let bitset_index = self.indexes[i];//get the bitset_index, recompute the new bit_set_index when lod = lod -1
		    let byte = self.bitset[i];
	        //let (x,y,z) = morton::decode(bitset_index, self.lod);
	        //let (new_x, new_y, new_z) = self.at_lod(x as i64, y as i64, z as i64, new_lod);
	        //let new_morton = morton::encode(new_x as u64, new_y as u64, new_z as u64, self.lod);
	        //let new_morton = morton::encode(new_x as u64, new_y as u64, new_z as u64, new_lod);
	        //println!("xyz:{},{},{} new_xyz:{},{},{}  bitset:{},  new_morton: {}",x,y,z,new_x,new_y,new_z,bitset_index, new_morton);
		    if byte > 0 {
			    parent_bitset.set_bit(bitset_index, true);
			    //parent_bitset.set_bit(new_morton, true);
		    }
	    }
	    parent_bitset
    }
    
    /*
    pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let new_lod = self.lod - 1;
	    let mut parent_bitset = Voxel::new(new_lod);
	    for i in range (0, len) {
	        let bitset_index = self.indexes[i];//get the bitset_index, recompute the new bit_set_index when lod = lod -1
		    let byte = self.bitset[i];
	        let (x,y,z) = morton::decode(bitset_index, self.lod);
	        let (new_x, new_y, new_z) = self.at_lod(x as i64, y as i64, z as i64, new_lod);
	        //let new_morton = morton::encode(new_x as u64, new_y as u64, new_z as u64, self.lod);
	        let new_morton = morton::encode(new_x as u64, new_y as u64, new_z as u64, new_lod);
	        println!("xyz:{},{},{} new_xyz:{},{},{}  bitset:{},  new_morton: {}",x,y,z,new_x,new_y,new_z,bitset_index, new_morton);
		    //if byte > 0 {
			    //parent_bitset.set_bit(bitset_index, true);
			    parent_bitset.set_bit(new_morton, true);
		    //}
	    }
	    parent_bitset
    }
    */
    
    
    pub fn clone(&self)->Voxel{
    	let copy_indexes = self.indexes.clone();
    	let copy_bitset = self.bitset.clone();
    	let copy_colors = self.colors.clone();
    	Voxel{bitset:copy_bitset, indexes:copy_indexes, colors:copy_colors, lod:self.lod, limit:self.limit, 
    	color_indexes:self.color_indexes.clone()
    	}
    }
    
     //get the x,y,z at given lod
    fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64).round() as i64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64).round() as i64;
    	println!("at lod {}: {},{},{}", new_lod, xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    
}

impl fmt::String for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     	let len = self.bitset.len();
     	let mut s = String::new();
    	for i in range (0, len) {
            let bitset_index = self.indexes[i];
            let line = format!("[{}]: {}\n",bitset_index, self.bitset[i]);
            s.push_str(line.as_slice());
         }
         write!(f, "{}",s)
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
