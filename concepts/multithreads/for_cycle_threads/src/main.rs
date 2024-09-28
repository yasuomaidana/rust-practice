use std::thread;

const NUMBER_OF_THREADS: i32 = 10;
fn main() {
    let mut threads = vec![];

    for i in 0..NUMBER_OF_THREADS {
        threads.push(thread::spawn(move || {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
