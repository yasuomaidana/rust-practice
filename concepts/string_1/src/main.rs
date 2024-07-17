use std::cmp::min;

fn cut_string(s: &str, start:i32, end:i32) -> &str {
    
    let end = min(s.len(), end as usize);
    s.get(start as usize..end).unwrap()
}

fn mut_cut_string(s: &mut String, start:i32, end:i32) -> String {
    let end = min(s.len(), end as usize);
    let start = start as usize;
    *s = s.drain(start..end).collect();
    s.to_string()
}

fn main() {
    let str_1 = "Hello, world!";
    println!("{}", cut_string(str_1, 2, 5));
    println!("After cutting (not mutable): {}", str_1);
    // print!("{}", mut_cut_string(&str_1, 2, 5));
    let mut str_2 = String::from("Hello, world! V2");

    println!("{}", mut_cut_string(&mut str_2, 2, 10));

    println!("After mutable cutting: {}", str_2);
}
