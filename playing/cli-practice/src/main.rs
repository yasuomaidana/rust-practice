use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use clap::Parser;


#[derive(Debug, Parser)]
struct Cli {
    #[clap(default_value = "fox")]
    pattern: String,
    #[clap(short, default_value_os  = "default_text.txt")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let pattern = format!("\x1b[0;31m{}\x1b[0m", &args.pattern);
    println!("Searching for pattern: {} in {}:\n", pattern, args.path.display());

    let content = fs::read_to_string(&args.path).expect("could not read file");
    let mut word_counter: HashMap<String, u32> = HashMap::new();
    for line in content.lines() {

        let line_to_search = line.to_ascii_lowercase();
        if line_to_search.contains(&args.pattern) {
            let line = line.replace(&args.pattern, &format!("\x1b[0;31m{}\x1b[0m", &args.pattern));
            println!("{}", line);
        }else {
            println!("{}", line);
        }

        let regex_pattern = regex::Regex::new(format!(r"\b{}\w+", &args.pattern).as_str()).unwrap();
        let matches = regex_pattern.find_iter(line_to_search.as_str());
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
