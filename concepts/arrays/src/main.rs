
fn main() {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let today = days[4];
    print!("First day {}\n", days[0]);
    println!("Today is {}", today);
    // print!("Invalid day {}", days[7]); it doesn't compile
    print!("Last day {}\n", days[6]);

    
    let multiples = [5; 5];
    println!("Multiples {:?}", multiples[0]);
    println!("Multiples {:?}", multiples);

}
