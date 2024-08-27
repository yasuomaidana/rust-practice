use std::thread;

fn main() {
    let x = 1;
    let second_thread = thread::spawn(move || {
        for i in 1..10 {
            if i == 5 {
                println!("x is {}", &x);
            }
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    println!("Still accessible and dropped {}", x);
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(2));
    }

    second_thread.join().unwrap(); // waits for second thread to finish
}
