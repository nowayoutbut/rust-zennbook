use std::{
    sync::{Arc, Mutex},
    thread,
};
fn main() {
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

            println!("{}", num);
        });

        handle.join().unwrap();
    }
}
