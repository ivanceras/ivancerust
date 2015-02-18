use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    let data = vec![1u32, 2, 3];
    let x = 5.0;
	let rc = Arc::new( vec![1u32, 2, 3]);
	
    for i in 0us..3 {
        let data_clone = data.clone();
        let rc_clone = rc.clone();
        Thread::spawn(move || {
            println!("data[{}]:{}",i,data_clone[i]);
            println!("arc[{}]:{}",i,rc_clone[i]);//this is compile error
            println!("x: {}",x);
        });
    }

    timer::sleep(Duration::milliseconds(50));
}

