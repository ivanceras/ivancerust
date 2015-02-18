use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    for i in 0us..2 {
        let data = data.clone();
        Thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    timer::sleep(Duration::milliseconds(50));
    for i in 0us..2{
        println!("data: {}", data.unlock().unwrap());
    }
}
