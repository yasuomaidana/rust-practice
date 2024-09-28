use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{sleep, spawn};

static NUMBER_OF_THREADS: i32 = 5;
fn main() {
    let (transmitter, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut threads = vec![];

    for id in 0..NUMBER_OF_THREADS {
        let thread_tx = transmitter.clone();

        let thread = spawn(move || {
            thread_tx.send(id).unwrap();
            sleep(std::time::Duration::from_millis(1000));
            println!("thread {} finished!", id);
        });
        threads.push(thread);
    }

    let mut ids = vec![];
    for thread in threads {
        thread.join().unwrap();
        ids.push(receiver.recv().unwrap());
    }

    println!("ids: {:?}", ids);
}
