use std::fs;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("Using input variables");
    println!("{:?}", args);

    let content = fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines(){
        if line.contains(&args.pattern){
            let line = line.replace(&args.pattern, &format!("\x1b[0;31m{}\x1b[0m", &args.pattern));
            println!("{}", line);
        }
    }
}
