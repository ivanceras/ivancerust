//test_index_value.rs
use index_value::IndexValue;

mod index_value;

fn main(){
	let mut indexes = vec![10,1,2,8,4,7,6,5,11,22,13,3,88,91,77,102];
	let mut index_value = IndexValue::new();
	for i in range(0, indexes.len()){
		let v = indexes[i];
		index_value.push(v);
	}
	let index = index_value.index_of(22);
	println!("index: {}", index);
}
