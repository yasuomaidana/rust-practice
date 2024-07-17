fn un_mutable_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}
fn array_mean(numbers: &[i32])-> f64 {
    let sum = un_mutable_sum(numbers);
    sum as f64 / numbers.len() as f64
}

fn mutable_sum(numbers: &mut [i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += *number;
        *number += sum;
    }
    sum
}
fn main() {
    let unmutable_array = [1,2,3,4];
    println!("Unmutable sum {}", un_mutable_sum(&unmutable_array));
    println!("Unmutable array {:?}, mean: {}", unmutable_array, array_mean(&unmutable_array));
    
    let mut mutable_array = [1,2,3,4];
    println!("Mutable sum {}", mutable_sum(&mut mutable_array));
    println!("Mutable array {:?}", mutable_array);
}
