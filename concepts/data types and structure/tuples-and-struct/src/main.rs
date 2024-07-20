
fn print_header(header: &str) {
    let total_len = 60;

    let text_len = header.len();
    let padding_len = (total_len - text_len)/2;
    let right_padding_len = total_len - text_len - padding_len;
    let left_padding = "=".repeat(padding_len);
    let right_padding = "=".repeat(right_padding_len);
    println!("{}{}{}{}", left_padding, header, right_padding, "\n");
}
fn print_footer() {
    let total_len = 60;
    let mut padding = String::from("=").repeat(total_len);
    padding.pop();
    padding.insert(0, '|');
    padding.pop();
    padding.insert(padding.len(), '|');
    println!("\n{}", padding);
}

fn main() {
    print_header("Tuples part 1: Accessing tuple elements");
    let tuple = ("Hi", 1, [1,2,3]);
    println!("Tuple: {:?}", tuple);
    println!("Accessing tuple elements by indexing: {}, {}, {:?}", tuple.0, tuple.1, tuple.2);
    let (text, number, list) = tuple;
    println!("Accessing tuple elements by destructuring: {}, {}, {:?}", text, number, list);
    print_footer();
    let tuple_vector = vec![("Hi", 1), ("Hello", 2)];

    print!("Iterating over tuple vector");
    println!("Iterating over tuple vector");
    for (text, number) in tuple_vector.iter() {
        println!("\tText part: {text}, Numeric part: {number}");
    }

}
