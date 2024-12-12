use std::fs;
use std::path::Path;

fn walk_path(path: &Path){
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_path(&path);
        } else {
            println!("{}", path.display());
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
