use std::fs::{File};
use std::io::{BufReader};

pub(crate) fn read_file(file_name: &str) -> BufReader<File> {
    let file = File::open(file_name).expect("Could not open file");
    BufReader::new(file)
}
