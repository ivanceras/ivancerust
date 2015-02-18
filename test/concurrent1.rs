use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    for i in 0us..3 {
        let data = data.clone();
        Thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
            println!("data[{}]:{}",i,data[i]);
        });
    }

    timer::sleep(Duration::milliseconds(50));
}

