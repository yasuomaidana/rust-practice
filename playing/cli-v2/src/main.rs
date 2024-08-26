mod cars;

use clap::Parser;
use crate::cars::{read_cars, read_from_file, show_cars};

#[derive(Parser)]
#[clap(name = "My RPN program",
    version = "1.0.0", author = "yo merenges")]
struct Options {
    #[clap(short, long)]
    cars: Option<String>,
    csv: Option<String>,
}
fn main() {
    let opts = Options::parse();
    let cars =
    match opts.csv {
        Some(csv) => read_from_file(&csv),
        None => read_cars(opts.cars.unwrap_or_default().as_str()),
    };
    show_cars(cars);
}
