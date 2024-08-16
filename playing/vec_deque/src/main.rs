use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::error::Error;
use rayon::prelude::*;

fn main() {
    // Verify command-line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: {} <input_file.txt> <subsequence>", std::env::args().next().unwrap());
        return;
    }
    let input_file_path = &args[0];
    let sub_sequence = &args[1];

    // read the file of dna sequences
    let contents = read_file(input_file_path).unwrap();
    let dna_sequences : Vec<&str> = contents.split("\n").map(|line| line.trim()).collect();

    // Search for the sub-sequence in parallel
    let found_sequences = dna_sequences.into_par_iter().filter_map(move |seq| {
        if contains_subsequence(seq, sub_sequence) {
            Some(seq)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    if found_sequences.is_empty() {
        println!("No matching sequence found.");
    } else {
        let output_file_path = "output.txt";
        let mut file = File::create(output_file_path).unwrap();
        for sequence in found_sequences {
            file.write_all(sequence.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        println!("Results written to: {}", output_file_path);
    }
}

fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn contains_subsequence(seq: &str, sub_seq: &str) -> bool {
    let seq_len = seq.len();
    let sub_seq_len = sub_seq.len();

    // Corner cases
    if sub_seq_len > seq_len {
        return false;
    }

    for i in 0..=seq_len - sub_seq_len {
        let window = &seq[i..i + sub_seq_len];
        if window == sub_seq {
            return true;
        }
    }
    false
}