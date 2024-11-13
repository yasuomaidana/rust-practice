use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args{
    #[clap(short, long, default_value = "file.txt")]
    file_name: PathBuf
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.file_name).expect("Unable to read file");
    let number_of_rows = file.lines().count();
    println!("Number of rows: {}", number_of_rows);
}
