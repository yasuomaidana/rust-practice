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
