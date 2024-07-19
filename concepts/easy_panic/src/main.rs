fn loop_and_panic(numbers: Vec<i32>) {

    for i in numbers {
        if i < 0 {
            panic!("{i} is negative");
        }
        println!("Number: {i}");
    }
}
fn main() {
    let numbers = vec![1, 2, 3, 4, -5];
    loop_and_panic(numbers);
}
