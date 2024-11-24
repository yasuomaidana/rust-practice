#[macro_use] mod macros;

fn main() {
    println!("Hello, world!");
    let a= find_min!(1, 2, 3, 4);
    println!("Min is {}", a);
    
}
