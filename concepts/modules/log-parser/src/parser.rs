use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;

fn get_reader(file_path: &str) -> Box<dyn std::io::BufRead> {
    let file = File::open(file_path).unwrap();
    match file_path.ends_with(".gz") {
        true => {
            let decompressor = GzDecoder::new(file);
            Box::new(std::io::BufReader::new(decompressor))
        }
        false => Box::new(std::io::BufReader::new(file)),
    }
}

pub fn read_buffer(file_path: &str) {
    let reader = get_reader(file_path);

    // Initialize variables for error rate calculation.
    let mut total_entries = 0;
    let mut error_entries = 0;
    let mut current_hour = None;

    let timestamp_regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}-\d{2})").unwrap();
    let error_keyword = "Error";

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                continue;
            }
        };
        // Extract timestamp from the log line.
        if let Some(captures) = timestamp_regex.captures(&line) {
            let timestamp = &captures[1];
            // Extract the date and hour part of the timestamp, assuming it's in "YYYY-MM-DD HH:MM:SS-ZZ" format.
            let date = &timestamp[0..10];
            let hour = &timestamp[11..13];
            let date_hour = format!("{}, Hour: {}", date, hour);

            // Check if the hour has changed.
            if current_hour != Some(date_hour.to_string()) {
                // Calculate and print error count for the previous hour.
                if let Some(prev_hour) = current_hour.take() {
                    println!("{prev_hour} - Error Count: {error_entries}");
                }

                // Reset counters for the new hour.
                error_entries = 0;
                current_hour = Some(date_hour.to_string());
            }

            // Check if the log entry contains an error.
            if line.contains(error_keyword) {
                error_entries += 1;
                total_entries += 1;
            }
        }
    }
    println!("Total error count for current log: {total_entries}");
}
