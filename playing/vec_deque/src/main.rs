use std::collections::VecDeque;
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
    let dna_sequences: Vec<&str> = contents.split("\n").map(|line| line.trim()).collect();

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

    let seq: Vec<char> = seq.chars().collect();

    // Check for empty sub-sequence or sub-sequence larger than the sequence
    if sub_seq_len == 0 || seq_len < sub_seq_len {
        return false;
    }

    let mut left_window: VecDeque<char> = VecDeque::new();
    let mut right_window: VecDeque<char> = VecDeque::new();

    // Initialize the first windows
    for i in 0..sub_seq_len {
        left_window.push_back(seq[i]);
        right_window.push_front(seq[seq_len - i - 1]);
    }

    // Check if the first windows match the sub_seq
    if compare_chars_string(&left_window, sub_seq) || compare_chars_string(&right_window, sub_seq) {
        return true;
    }

    for i in 0..(seq_len - sub_seq_len) {
        // Move the windows by shifting in the next char and dropping the oldest char
        left_window.push_back(seq[i + sub_seq_len]);
        left_window.pop_front();

        right_window.push_front(seq[seq_len - i - 1 - sub_seq_len]);
        right_window.pop_back();

        // Check for match
        if compare_chars_string(&left_window, sub_seq) || compare_chars_string(&right_window, sub_seq) {
            return true;
        }
    }
    false
}

fn compare_chars_string(chars: &VecDeque<char>, sub_sequence: &str) -> bool {
    let mut sub_sequence_iter = sub_sequence.chars();

    for (a, b) in chars.iter().zip(sub_sequence_iter.by_ref()) {
        if *a != b {
            return false;
        }
    }
    sub_sequence_iter.next().is_none()
}