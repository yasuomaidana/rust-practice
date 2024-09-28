use std::sync::Mutex;

fn main() {
    let mutex_int = Mutex::new(5);
    println!("Starting mutex value: {:?}", mutex_int);
    { // New scope
        let mut muter_changer = mutex_int.lock().unwrap();
        println!("Inner scope mutex value: {}", &muter_changer);
        println!("Mutex variable value: {:?}", mutex_int);
        *muter_changer += 1; // modifying mutex value
    }// drop muter_changer is called but mutex_int remains

    println!("Values change remains even after scope, \n\tnew mutex value {:?}", mutex_int);
    println!("Only value {}", *mutex_int.try_lock().unwrap());
    match mutex_int.try_lock() {
        Ok(value) => {
            println!("Value using matching: {value}");
        }
        Err(_) => {
        }
    };
    let _unused_value = mutex_int.lock().unwrap();

    match mutex_int.try_lock() {
        Ok(value) => {
            println!("Value using matching: {value}");
        }
        Err(_) => {
            println!("Not available"); // It is not available because it is being used
            // by _unused_value
        }
    };

}
