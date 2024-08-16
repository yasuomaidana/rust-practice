use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, Read, Write};
use rayon::prelude::*;

fn main() {
    // Verify command-line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: {} <input_file.txt> <subsequence>", std::env::args().next().unwrap());
        return;
    }

    let input_file_path = &args[0].trim();
    println!("Searching for sub-sequence in file: {}", input_file_path);
    let sub_sequence = &args[1].trim();
    println!("Sub-sequence to search: {}", sub_sequence);

    // read the file
    let contents = read_file(input_file_path).unwrap();

    //split the file data into dna sequences
    let dna_sequences : VecDeque<&str> = contents.split("\n").into_iter().filter_map(|line| Some(line.trim())).collect();

    // Search for the sub-sequence in parallel
    let found_sequences = dna_sequences.into_par_iter().filter_map(|seq| {
        if seq.contains(sub_sequence) {
            Some(seq)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    if found_sequences.is_empty() {
        println!("No matching sequence found.");
    } else {
        let output_file_path = "output.txt";
        let mut  file = File::create(output_file_path).unwrap();
        for sequence in found_sequences {
            println!("{}", sequence);
            file.write_fmt(format_args!("{}\n", sequence)).unwrap();
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