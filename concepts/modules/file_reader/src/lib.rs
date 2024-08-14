pub mod text_reader;

use std::fs;
use std::io::BufRead;




pub struct TextFile {
    file_name: String,
    lines: Vec<String>,
}

impl TextFile {
    pub fn new(file_name: &str) -> TextFile {
        let lines = _read_file(file_name);
        TextFile {
            file_name: file_name.to_string(),
            lines,
        }
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn read_from_console() -> TextFile {
        let mut input = String::new();
        println!("Enter file name: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        TextFile::new(input.trim())
    }

}

/// Reads the contents of a file and returns its lines as a vector of strings.
///
/// # Arguments
///
/// * `file_name` - A string slice that holds the name of the file to be read.
///
/// # Returns
///
/// A `Vec<String>` containing the lines of the file.
///
/// # Panics
///
/// This function will panic if the file cannot be read.
///
/// # Examples
///
/// ```ignore
/// let lines = _read_file("example.txt");
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
fn _read_file(file_name: &str) -> Vec<String> {
    fs::read(file_name)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
