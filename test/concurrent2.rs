use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;

fn main() {
    let data = Arc::new(vec![1u32, 2, 3]);
    let x = 5.0;

    for i in 0us..3 {
        let data_clone = data.clone();
        Thread::spawn(move || {
            println!("data[{}]:{}",i,data_clone[i]);
            println!("x: {}",x);
        });
    }

    timer::sleep(Duration::milliseconds(50));
}
