//voxel.rs
use std::fmt;
use color::Color;
use morton::morton;
use std::num::Float;

pub struct Voxel{
    lod:u8,
    limit:u64,
    bitset:Vec<u8>,
    indexes:Vec<u64>,
    pub colors:Vec<Color>
}

impl Voxel{

    pub fn new(lod:u8)->Voxel{
        let mut bitset = Vec::new();
        let mut indexes = Vec::new();
        let mut colors = Vec::new();
        Voxel{lod:lod, limit:1<<lod, bitset:bitset, indexes:indexes, colors:colors}
     }

    fn set_bit(&mut self, bit_index:u64, val:bool){
	    let byte_index = (bit_index / 8) as u64;
        let remainder = bit_index % 8;
        let mut bitset_index = self.index_of(byte_index);
        if bitset_index < 0 {
            self.indexes.push(byte_index);
            self.bitset.push(0);
            bitset_index = self.index_of(byte_index);
        }
        let byte = self.bitset[bitset_index as usize];
        if val {
            let or_byte = 1 << remainder;
            let new_byte = byte | or_byte;
            self.bitset[bitset_index as usize] = new_byte;
        }
	    
    }
    
      pub fn set_bit_at_loc(&mut self, x:u64, y:u64, z:u64, val:bool){
        let bit_index = morton(x,y,z,self.lod);
	    let byte_index = (bit_index / 8) as u64;
        let remainder = bit_index % 8;
        let mut bitset_index = self.index_of(byte_index);
        if bitset_index < 0 {
            self.indexes.push(byte_index);
            self.bitset.push(0);
            let r = 255 - ((x as f64 / self.limit as f64) * 255.0).round() as u8;
            let g = 255 - ((y as f64 / self.limit as f64) * 255.0).round() as u8;
            let b = 255 - ((z as f64 / self.limit as f64) * 255.0).round() as u8;
            self.colors.push(Color{r:r, g:g, b:b});
            bitset_index = self.index_of(byte_index);
        }
        let byte = self.bitset[bitset_index as usize];
        if val {
            let or_byte = 1 << remainder;
            let new_byte = byte | or_byte;
            self.bitset[bitset_index as usize] = new_byte;
        }
	    
    }
    
    pub fn index_of(&self, value:u64)->i64{
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
        let mut bitset_index = self.index_of(byte_index);
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
	    let byte_index = (bit_index / 8) as u64;
        let remainder = bit_index % 8;
        let mut bitset_index = self.index_of(byte_index);
        if bitset_index < 0 {
        	return Color{r:0,g:0,b:0};
        }
        self.colors[bitset_index as usize].clone()
    }
    
    pub fn show_indexes(&self){
        println!("There are {} occupied", self.indexes.len());
        for i in self.indexes.iter(){
            println!("{}",i);
        }
    }
    
    //get the parent bit_set of this bitset
    pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let mut parent_bitset = Voxel::new(self.lod-1);
	    for i in range (0, len) {
	        let bitset_index = self.indexes[i];
		    let byte = self.bitset[i];
		    if byte > 0 {
			    parent_bitset.set_bit(bitset_index, true);
		    }
	    }
	    parent_bitset
    }
    
    pub fn clone(&self)->Voxel{
    	let copy_indexes = self.indexes.clone();
    	let copy_bitset = self.bitset.clone();
    	let copy_colors = self.colors.clone();
    	Voxel{bitset:copy_bitset, indexes:copy_indexes, colors:copy_colors, lod:self.lod, limit:self.limit}
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
