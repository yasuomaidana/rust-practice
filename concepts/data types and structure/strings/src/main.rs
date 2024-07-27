fn print_str(s: &str) {
    println!("{}", s);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn print_string_ref(s: &String) {
    println!("{}", s);
}


fn main() {
    let s = "Hello, World!";
    print_str(s);
    println!("Second time, &str {}", s);
    let s = String::from("Hello, World!");
    print_string(s);
    // print_string_ref(&s); // Error: cannot borrow `s` as mutable, as it is not declared as mutable
    let s = String::from("Hello, World!");
    print_string_ref(&s);

    let s = "A random text, with commas and dots. \nAnd a new line!".to_string();
    println!("Manipulating the string: \n{}", s);
    // print_string_ref(&s); // Error: cannot borrow `s` as mutable, as it is not declared as mutable
    let words:Vec<&str> = s.split_whitespace().collect();
    println!("Words: {:?}", words);
    let sentences:Vec<&str> = s.split('\n').collect();
    println!("Sentences: {:?}", sentences);

    let reversed = s.chars().rev().filter(|&x| x!='\n').collect::<String>();
    println!("Reversed: {}", reversed);
    println!("Original: {}", s);
}
