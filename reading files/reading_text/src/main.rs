use std::collections::{ HashSet };
use std::io::BufRead;
use crate::file_reader::read_file;

mod file_reader;
mod regex_matcher;

fn main() {
    let file_name = "example.log";
    let reader = read_file(file_name);
    println!("Reading file: {}", file_name);
    let mut ips:HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let ip = regex_matcher::find_ip(&line);
        match ip {
            Some(ip) => {
                ips.insert(ip);
            },
            None => {}
        }
    }
    println!("Unique IPs found: {:?}", ips);
}
