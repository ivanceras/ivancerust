//test flip bytes
fn main(){
	let rem = 0;
	let tmp_byte:u8 = 1 << rem;
    let and_byte = !tmp_byte;//flip the bits
    println!("bytes {}:  {}",tmp_byte, and_byte);
}
