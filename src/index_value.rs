//IndexValue.rs

pub struct IndexValue{
	keys:Vec<u64>, //sorted values
	values:Vec<u64>, // original index arrangement
	sorted:bool
}

impl IndexValue{
	
	
	pub fn new()->IndexValue{
		IndexValue{keys:Vec::new(), values:Vec::new(), sorted:false}
	}
	
	pub fn push(&mut self, key:u64){
		self.keys.push(key);
		self.sorted = false;
	}
	
	//put the previous index to values array
	fn sort(&mut self){
		let mut sorted = self.keys.clone();
		sorted.sort();
		for i in range(0, sorted.len()){
			let value = sorted[i];
			let orig_index = self.original_index_of(value);
			self.values.push(orig_index as u64);
		}
		self.keys = sorted;
	}
	
	fn original_index_of(&self, val: u64)->usize{
		let pos = self.keys.iter().position(|v| *v == val);
		match pos {
		    Some(p) => return p,
		    None => return -1
		}
	}
	
	//uses binary search
	pub fn index_of(&mut self, search:u64)->i64{
		if !self.sorted {//lazy sorting
			self.sort();
			self.sorted = true;
		}
		let mut first = 0;
		let mut last = self.keys.len() - 1;
		let mut middle = (first+last)/2;
		println!("searching...");
		let mut cnt = 0;
		loop {
			if  self.keys[middle] < search {
				first = middle + 1;    
			}
			else if  self.keys[middle] == search  {
			 	return self.values[middle] as i64;
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
	
}
