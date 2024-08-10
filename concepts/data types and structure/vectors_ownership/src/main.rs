
fn ownership() {
    let s = vec![1,2,3];
    let slice = &s[1..];
    println!("{:?}", slice);
}

fn modifiable(){
    let mut numbers = vec![4,5,6];
    let slice = &mut numbers[..];
    slice[0] = 10;
    println!("{:?}", slice);
}

fn splitting_a_vector(){
    println!("Splitting a vector");
    let mut numbers = vec![1,2,3,4,5,6,7,8,9,10];
    let (left, right) = numbers.split_at_mut(5);
    println!("{:?}", left);
    println!("{:?}", right);
}
fn main() {
    ownership();
    modifiable();
    splitting_a_vector();

    let sentence = String::from("The quick brown fox jumps over the lazy dog");
    let (left, right) = sentence.split_at(19);
    println!("{:?}", left);
    println!("{:?}", right);

    let sentence = String::from("The quick brown fox jumps over the lazy dog");
    let split = sentence.rsplit("fox").collect::<Vec<&str>>();
    println!("{:?}", split);

    let sentence = String::from("The quick brown fox jumps over the lazy dog");
    let split = sentence.split("fox").collect::<Vec<&str>>();
    println!("{:?}", split);

    let sentence = String::from("The quick brown fox jumps over the lazy dog");
    let split = sentence.split_inclusive("fox").collect::<Vec<&str>>();
    println!("{:?}", split);

}
