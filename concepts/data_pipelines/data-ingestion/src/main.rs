fn ingest_vector(data: &mut Vec<i32>){
    (1..=3).for_each(|x| data.push(x));
}
fn main() {
    let mut data = Vec::new();
    ingest_vector(&mut data);
    println!("{:?}", data);
}
