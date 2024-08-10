use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the poem from a text file
    let filename = "testing.txt";
    let file = File::open(filename).expect("Could not open file");
    let mut reader = io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Could not read file");


    // Create a regex to match the last word of every sentence
    let re = Regex::new(r"\b(\w+)\b\W*$").expect("Could not create regex");

    // Split the poem into verses and store the last words of each sentence in a list
    let verses: Vec<Vec<&str>> = contents
        .split("\n")
        .map(|verse| {
            let words: Vec<&str> = re.captures_iter(verse).map(|caps| caps.get(1).map_or("",|m| m.as_str())).collect();
            words
        })
        .fold(Vec::new(), |mut acc, words| {
            if words.is_empty() {
                acc.push(Vec::new());
            }
            else{
                if acc.is_empty() {
                    acc.push(Vec::new());
                }
                let last = acc.len();
                acc.get_mut(last - 1).unwrap().extend(words);
            }
            acc
        });

    // Print the result
    for verse in verses {
        println!("{:?}", verse);
    }

    Ok(())
}
