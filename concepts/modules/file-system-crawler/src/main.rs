use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn check_file(path: &Path) {
    let file = File::open(path);
    let mut buffer = [0; 1024];
    let mut reader = BufReader::new(file.unwrap());
    let bytes_read = reader.read(&mut buffer).unwrap();

    if bytes_read > 0 && std::str::from_utf8(&buffer[..bytes_read]).is_ok() {
        println!("Plain text file: {}", path.display());
    } else {
        println!("Binary file: {}", path.display());
    }
}

fn walk_path(path: &Path) {
    if !path.is_dir() {
        check_file(path);
        return;
    }
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.to_str().unwrap().contains("target") {
            continue;
        }
        if path.is_dir() {
            walk_path(&path);
        } else {
            check_file(&path);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        println!("{} does not exist", path.display());
        return;
    }
    walk_path(path);
}
