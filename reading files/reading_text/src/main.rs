use std::io::BufRead;
use crate::file_reader::read_file;

mod file_reader;

fn main() {
    let file_name = "example.log";
    let reader = read_file(file_name);
    println!("Reading file: {}", file_name);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
