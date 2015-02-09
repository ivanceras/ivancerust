//voxel.rs
use std::fmt;

pub struct Voxel{
    bitset:Vec<u8>,
    indexes:Vec<u64>
}

impl Voxel{

    pub fn new()->Voxel{
        let mut bitset = Vec::new();
        let mut indexes = Vec::new();
        Voxel{bitset:bitset, indexes:indexes}
     }

    pub fn set_bit(&mut self, bit_index:u64, val:bool){
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
    
    pub fn show_indexes(&self){
        println!("There are {} occupied", self.indexes.len());
        for i in self.indexes.iter(){
            println!("{}",i);
        }
    }
    
    /*pub fn display_bitset(&self){
        println!("Showing bitsets....");
        assert!(self.indexes.len() == self.bitset.len());
        let len = self.bitset.len();
        let mut count:u64 = 0;
        for i in range (0, len) {
            let bitset_index = self.indexes[i];
            println!("[{}]: {}",bitset_index, self.bitset[i]);
            count += count_bits(self.bitset[i]) as u64;
        }
        println!("There are total of {} voxels",count);
    }
    */
    
    //get the parent bit_set of this bitset
    pub fn parent(&self)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let mut parent_bitset = Voxel::new();
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
    	Voxel{bitset:copy_bitset, indexes:copy_indexes}
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
