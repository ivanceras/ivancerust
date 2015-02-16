//test_sort.rs

fn main(){
	let mut indexes = vec![10, 2, 5, 12, 4, 1];
	//indexes =      0   1  2   3  4  5
	let orig = indexes.clone();
	let values:Vec<usize> = Vec::with_capacity(indexes.len());
	indexes.sort();
	for i in range(0, indexes.len()){
		let value = indexes[i];
		let orig_index = index_of(&orig, value);
		println!("n[{}]: {}",orig_index, value);
	}
}


fn index_of(vector:&Vec<i64>, val : i64)->usize{
	let pos = vector.iter().position(|v| *v == val);
	match pos {
        Some(p) => return p,
        None => return -1
    }
}

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            (*std::intrinsics::get_tydesc::<T>()).name
        };
    println!("{}", type_name);
}
