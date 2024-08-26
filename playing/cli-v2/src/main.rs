use clap::Parser;

#[derive(Parser)]
#[clap(name = "My RPN program",
    version = "1.0.0", author = "yo merenges")]
struct Options {
    verbose: bool,
    debug: bool,
}
fn main() {
    println!("Hello, world!");
}
