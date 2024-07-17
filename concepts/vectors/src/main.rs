fn main() {
    let three_elemnts = vec![1, 2, 3];
    let one_elements = vec![1; 5];
    println!("Three elemnts vectors: {:?}", three_elemnts);
    println!("One elemnts vectors: {:?}", one_elements);
    println!("Pushing elemnt three elements");
    let mut mutable_vector = vec![1, 2, 3];
    println!("Before pushing: {:?}", mutable_vector);
    mutable_vector.push(4);
    println!("After pushing: {:?}", mutable_vector);

    println!("Accesing to element index 3:{:}", mutable_vector[3]);
    // println!("Accesing to element index 4:{:?}", mutable_vector[4]);

    print!("Removing last element: ");
    mutable_vector.pop();
    println!("{:?}", mutable_vector);

    print!("Removing element at index 1: ");
    mutable_vector.remove(1);
    println!("{:?}", mutable_vector);

    print!("Inserting element at index 1: ");
    mutable_vector.insert(1, 2);
    println!("{:?}", mutable_vector);

    print!("Modifying element at index 2 by adding 5: ");
    mutable_vector[2] += 5;
    println!("{:?}", mutable_vector);

    println!("Iterating over vector: ");
    for i in &mutable_vector {
        print!("{:} ", i);
    }
    println!();

    println!("Iterating over vector with index: ");
    for (i, v) in mutable_vector.iter().enumerate() {
        if (i + 1) == mutable_vector.len() {
            print!("(I:{:}, V:{:}) ", i, v);
        } else {
            print!("(I:{:}, V:{:}),", i, v);
        }
    }
    println!();

    print!("Vector length: ");
    println!("{:?}", mutable_vector.len());

    print!("Clearing vector: ");
    mutable_vector.clear();
    println!("{:?}", mutable_vector);
}
