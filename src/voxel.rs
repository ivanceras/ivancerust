//voxel.rs
use std::fmt;
use color::Color;
use morton::Morton;
use std::num::Float;

pub struct Voxel{
    pub lod:u8,
    limit:u64,
    pub indexes:Vec<u64>,
    colors:Vec<Color>,
    density:f64,
    
}

impl Voxel{

    pub fn new(lod:u8)->Voxel{
        Voxel{
		    lod:lod, 
		    limit:1<<lod, 
		    colors:Vec::new(), 
		    indexes:Vec::new(),
		    density:1.0
		}
     }

    
   
    //TODO need to set an unset, bit and color as well, colors should have its own storage per bits, not per 8bits
    pub fn set_bit_at_loc(&mut self, x:u64, y:u64, z:u64, val:bool, color:Color){
    	let morton = Morton::new(self.lod);
        let bit_index = morton.encode(x,y,z);
	    self.set_color(bit_index,color);
	    
    }
    
    pub fn set_color(&mut self, bit_index:u64, color:Color){
		let color_index = self.index_of_color(bit_index);
		if color_index < 0 {
			self.colors.push(color);
			self.indexes.push(bit_index);
		}else{
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
		let morton = Morton::new(self.lod);
		while first <= last {
			if  morton.a_gt_b(search, self.indexes[middle as usize]) {
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
    	let morton = Morton::new(self.lod);
    	let bit_index = morton.encode(x as u64, y as u64, z as u64);
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
    	let morton = Morton::new(self.lod);
        let m = morton.encode(x as u64, y as u64, z as u64);
        self.get_color(m)
    }
    
    
    
    pub fn build_voxel_lod(&self, new_lod:u8)->Voxel{
	    println!("Getting the parent bitset...");
	    let len = self.indexes.len();
	    let mut linear_indexes = Vec::new();
	    let mut linear_voxel_colors = Vec::new();//store first before getting the average, array of up to 8 colors
	    let mut linear_morton = Vec::new();
	    let new_limit = 1 << new_lod;
	    let morton = Morton::new(self.lod);
	    let new_morton = Morton::new(new_lod);
	    let mut percentage = 0;
	    for i in range (0, len) {
	    	let new_percentage = ((i as f64 / len as f64) * 100.0).round() as u64;
	    	if new_percentage != percentage {
    			println!("lod{}, {}%",new_lod, new_percentage);
    		}
	    	percentage = new_percentage;
	        let bitset_index = self.indexes[i];//get the bitset_index, recompute the new bit_set_index when lod = lod -1
	        let (x,y,z) = morton.decode(bitset_index);
	        //let (new_x, new_y, new_z) = self.at_lod(x as i64, y as i64, z as i64, new_lod);//use ceiling since approximation should include whatever is the least
			let r = 256 - ((x as f64 / new_limit as f64) * 255.0).round() as u8;
			let g = 256 - ((y as f64 / new_limit as f64) * 255.0).round() as u8;
			let b = 256 - ((z as f64 / new_limit as f64) * 255.0).round() as u8;
			let color = Color::new(r,g,b,255);
			//let new_morton_index = new_morton.encode(new_x as u64, new_y as u64, new_z as u64);
			let new_morton_index = morton.at_lod(bitset_index, new_lod);
			let linear_index = new_morton.linear_index(new_morton_index);
			let iter_index = self.iterative_index_of(&linear_indexes, linear_index);
			if iter_index < 0 {
				linear_indexes.push(linear_index);
				linear_morton.push(new_morton_index);
	    		let mut voxel_color = Vec::new();
				voxel_color.push(color);
				linear_voxel_colors.push(voxel_color);

			}
			else{
				linear_indexes[iter_index as usize] = linear_index;
				linear_morton[iter_index as usize] = new_morton_index;
				linear_voxel_colors[iter_index as usize].push(color);

			}
	    }
	    
	    let original_indexes = linear_indexes.clone(); 
	    linear_indexes.sort();
  	    let mut parent_bitset = Voxel::new(new_lod);
		
	    for s in range(0, linear_indexes.len()){
	     	let linear_index = linear_indexes[s];
	     	let orig_index = self.iterative_index_of(&original_indexes, linear_index);
	    	let morton = linear_morton[orig_index as usize];
	    	
	    	let ref voxel_colors = linear_voxel_colors[orig_index as usize];
	    	//calculate the voxel colors
	    	let mut r_total = 0u64;
	    	let mut g_total = 0u64;
	    	let mut b_total = 0u64;
	    	let mut cnt = 1u64;
			for i in 0..voxel_colors.len(){
				let ref c = voxel_colors[i];
				//println!("color: {}",c);
				r_total += c.r as u64;
				g_total += c.g as u64;
				b_total += c.b as u64;
				cnt += 1;	
			}
			let mut r_ave = (r_total as f64/cnt as f64).round() as u8;
	    	let mut g_ave = (g_total as f64/cnt as f64).round() as u8;
	    	let mut b_ave = (b_total as f64/cnt as f64).round() as u8;
	    	
			let color = Color::new(r_ave, g_ave, b_ave, 255);
	    	parent_bitset.set_color(morton, color);
	    }
	    parent_bitset
    }
    
    
    pub fn clone(&self)->Voxel{
    	Voxel{
    	    colors:self.colors.clone(), 
    	    lod:self.lod, 
    	    limit:self.limit, 
    	    indexes:self.indexes.clone(),
    	    density:self.density.clone()
    	}
    }
    
     //get the x,y,z at given lod
     /*
    pub fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	let limit = 1 << self.lod;
    	let new_limit = 1 << new_lod;
    	let xnew = (x as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	let ynew = (y as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	let znew = (z as f64 * new_limit as f64 / limit as f64 ).round() as i64;
    	//println!("at lod {}: {},{},{}", new_lod, xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    */
    
    pub fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
    	let div = 1 << (self.lod - new_lod);//same as (1 << self.lod) / (1 << new_lod)
    	let xnew = (x as f64 / div as f64).round() as i64;
    	let ynew = (y as f64 / div as f64).round() as i64;
    	let znew = (z as f64 / div as f64).round() as i64;
    	//println!("at lod {}: {},{},{}", new_lod, xnew, ynew, znew);
    	(xnew, ynew, znew)
    }
    
    /*
	pub fn at_lod(&self, x:i64, y:i64, z:i64, new_lod:u8)->(i64, i64, i64){
		let morton = Morton::new(self.lod);
		morton.point_at_lod(x as u64,y as u64,z as u64, new_lod)
	}
	*/
	
    
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
