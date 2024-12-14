mod parser;
use parser::read_buffer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <path>");
        std::process::exit(1);
    }
    // macOs usage example: cargo run --release -- /var/log/system.log
    let path = &args[1];
    read_buffer(path);
}
