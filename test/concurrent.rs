use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut vec_array = Vec::new();
    println!("populating arrays..");
    for m in range(0,75_000){//50K threads in ofiz, can reach 74K in gaming
        vec_array.push(m);
    }
    let len = vec_array.len();
    let data = Arc::new(Mutex::new(vec_array));
    println!("Spawning threads...");
    for i in range(0,len) {
        let tx_clone = tx.clone();
        let data_clone = data.clone();
        Thread::spawn(move || {
            let array = data_clone.lock().unwrap();
            let val = array[i];
            tx_clone.send((i,val));
        });
    }
    
    for j in range(0,len) {
       let (pos,ret) = rx.recv().ok().expect("Could not recieve answer");
       println!("ret[{}]: {}",pos,ret);
   }
}
