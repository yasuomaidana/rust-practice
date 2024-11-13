use std::path::PathBuf;
use clap::{arg, Parser};
use rand::seq::SliceRandom;
use regex::Regex;

#[derive(Parser)]
#[command(name = "file_mixer")]
#[command(about = "A CLI tool to check if a file contains a specific string")]
struct Cli {
    #[arg(short, long, default_value = "at_text.txt")]
    file_name: PathBuf,
    #[arg(short, long, default_value = "@")]
    search_string: String,
}

fn split_keep(r: &Regex, text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut first = true;
    for mat in r.find_iter(text) {
        let review = &text[last..mat.start()];
        if first {
            first = false;
            continue;
        }
        result.push(review.to_string());
        last = mat.start();
    }
    if last < text.len() {
        result.push(text[last..].to_string());
    }

    result
}

fn main() {
    let args = Cli::parse();
    let file_name = args.file_name;
    let file_content = std::fs::read_to_string(file_name).unwrap();
    let separator = args.search_string;
    let separator = format!("{separator}\\w+:");
    let re = Regex::new(&separator).unwrap();
    let mut reviews:Vec<String> = split_keep(&re, &file_content);
    let mut rng = rand::thread_rng();
    reviews.shuffle(&mut rng);
    for review in reviews {
        println!("{}", review);
    }

}
