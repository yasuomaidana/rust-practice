use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn try_to_read(file_name: &str) {
    let file = File::open(file_name);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found: {}", file_name);
            return;
            },
            _ => {
                println!("Problem opening the file: {file_name}\nError: {error}");
                return;
            }
        },
    };

}
fn main() {
    try_to_read("foo.txt");
    try_to_read("valid_file.txt");
}
