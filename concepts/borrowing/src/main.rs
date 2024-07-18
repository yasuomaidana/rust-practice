use std::vec;
mod footer_header;
use footer_header::{print_footer, print_header};

fn own_integer(int: i32)->(){
    print_header();
    println!("Borrowed int {int}");
    print_footer();
}

fn own_string(string: String)->(){
    print_header();
    println!("Borrowed string {string}");
    println!("After borrowing string: {string}, it won't exist anymore");
    print_footer();
}

fn own_vector(vector: Vec<i32>)->(){
    print_header();
    println!("Borrowed vector {:?}",vector);
    print_footer();
}
fn main() {
    let mut my_vect = vec![1, 2, 3, 4, 5];
    let my_int = 6;
    let my_string = String::from("Hello, world!");

    own_integer(my_int);
    println!("After calling own_integer: {my_int}");
    own_string(my_string);
    //println!("After calling own_string: {my_string}");
    own_vector(my_vect);
    //println!("After calling own_vector: {:?}",my_vect);

}
