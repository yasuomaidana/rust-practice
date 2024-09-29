use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let num_threads = 10;

    let threads = (0..num_threads).map(|_| {
        let counter_clone = Arc::clone(&counter);
        thread::spawn(move || {
            sleep(Duration::from_millis(1000));
            let counter = counter_clone.try_lock();
            match counter {
                Ok(mut num) => {
                    *num += 1;
                },
                Err(_) => {
                    println!("Thread failed to acquire lock");
                }
            }
        })
    });

    threads.for_each(|t| {
        t.join().unwrap();
    });

    println!("Result: {}", *counter.lock().unwrap());
}
