use crate::read_lines;

pub trait ReadData {
    fn read_data(&mut self) -> Option<char>;
}

pub struct FileReader {
    string_buffer: Vec<String>,
    current_chars: Vec<char>,
    new_line: bool,
    starting: bool,
}

impl FileReader {
    pub fn new(file_name: String) -> Self {
        FileReader {
            string_buffer: read_lines(file_name).unwrap(),
            current_chars: vec![],
            new_line: false,
            starting: true,
        }
    }
}

impl ReadData for FileReader {
    fn read_data(&mut self) -> Option<char> {
        if self.current_chars.len() == 0 {
            if self.string_buffer.len() == 0 {
                return None;
            }
            let line = self.string_buffer.remove(0);
            self.current_chars = line.chars().collect();
            self.new_line = true;
        }
        {
            if self.new_line {
                if self.starting {
                    self.starting = false;
                    self.new_line = false;
                } else {
                    self.new_line = false;
                    return Some('\n');
                }
            }
        }

        Some(self.current_chars.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::delete_file;
    use super::*;

    #[test]
    fn reading_text_file() {
        if Path::new("text.txt").exists() {
            delete_file("text.txt").expect("Unable to delete file");
        }
        fs::write("text.txt", "hello\nbye").expect("Unable to write file");

        let mut file_reader = FileReader::new("text.txt".to_string());

        assert_eq!(file_reader.read_data().unwrap(), 'h');
        assert_eq!(file_reader.read_data().unwrap(), 'e');
        assert_eq!(file_reader.read_data().unwrap(), 'l');
        assert_eq!(file_reader.read_data().unwrap(), 'l');
        assert_eq!(file_reader.read_data().unwrap(), 'o');
        assert_eq!(file_reader.read_data().unwrap(), '\n');
        assert_eq!(file_reader.read_data().unwrap(), 'b');
        assert_eq!(file_reader.read_data().unwrap(), 'y');
        assert_eq!(file_reader.read_data().unwrap(), 'e');
        assert_eq!(file_reader.read_data(), None);

        delete_file("text.txt").unwrap()
    }
}