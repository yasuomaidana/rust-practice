use std::collections::HashMap;
use std::fs;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("Using input variables");
    println!("{:?}", args);

    let content = fs::read_to_string(&args.path).expect("could not read file");
    let mut word_counter: HashMap<String, u32> = HashMap::new();
    for line in content.lines() {
        if line.to_ascii_lowercase().contains(&args.pattern) {
            let line = line.replace(&args.pattern, &format!("\x1b[0;31m{}\x1b[0m", &args.pattern));
            println!("{}", line);
        }
        let regex_pattern = regex::Regex::new(r"child\w+").unwrap();
        let matches = regex_pattern.find_iter(line);
        for match_ in matches {
            word_counter.entry(match_.as_str().to_string()).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    println!("\n************************************\n");
    println!("The following words containing {} were found:", &args.pattern);
    let max_count = word_counter.values().max().or(Some(&0)).unwrap();
    for (word, count) in word_counter.iter() {
        if count == max_count {
            println!("\x1b[0;32m{}: {}\x1b[0m", word, count);
        } else {
            println!("{}: {}", word, count);
        }
    }
}
