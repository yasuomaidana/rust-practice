use reader::char_to_binary;
use reader::reader::ReadData;


pub fn encode_char(c: char, previous_bit: &mut u8) -> Vec<u8> {
    let binary = char_to_binary(c);
    let mut encoding = Vec::new();
    for bite in binary.chars() {
        let bit = bite.to_digit(2).unwrap() as u8;
        if bit == 0 {
            encoding.push(1 - *previous_bit);
            encoding.push(*previous_bit)
        } else {
            encoding.push(*previous_bit);
            *previous_bit = 1 - *previous_bit;
            encoding.push(*previous_bit);
        }
    }
    encoding
}

pub fn generate_encoding(data_reader: &mut impl ReadData, initial_bit: u8) -> Vec<u8> {
    let mut encoding = Vec::new();
    let mut previous_bit = initial_bit;
    while let Some(c) = data_reader.read_data() {
        encoding.extend(encode_char(c, &mut previous_bit));
    }
    encoding
}

struct Encoder {
    data_reader: Box<dyn ReadData>,
    previous_bit: u8,
}

impl Encoder {
    pub fn new(data_reader: impl ReadData + 'static, initial_bit: u8) -> Self {
        Encoder {
            data_reader: Box::new(data_reader),
            previous_bit: initial_bit,
        }
    }

    pub fn encode_char(&mut self) -> Vec<u8> {
        match self.data_reader.read_data() {
            None => {
                Vec::new()
            }
            Some(char) => {
                encode_char(char, &mut self.previous_bit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;
    use reader::delete_file;
    use reader::reader::FileReader;

    #[test]
    fn test_encoding_blank_space() {
        let mut previous_bit = 1;
        let blank_encoded = encode_char(' ', &mut previous_bit);
        assert_eq!(blank_encoded, vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(previous_bit, 0);
    }

    #[test]
    fn test_encoding_text() {
        if Path::new("text.txt").exists() {
            delete_file("text.txt").expect("Unable to delete file");
        }
        fs::write("text.txt", "a\n ").expect("Unable to write file");
        println!("{}", char_to_binary('a'));
        println!("{}", char_to_binary('\n'));
        println!("{}", char_to_binary(' '));
        let mut file_reader = FileReader::new("text.txt".to_string());
        let encoding = generate_encoding(&mut file_reader, 1);
        println!("{:?}", encoding);
    }

    #[test]
    fn test_encoder() {
        if Path::new("text.txt").exists() {
            delete_file("text.txt").expect("Unable to delete file");
        }
        fs::write("text.txt", " ").expect("Unable to write file");
        let file_reader = FileReader::new("text.txt".to_string());
        let mut encoder = Encoder::new(file_reader, 1);
        let encoding = encoder.encode_char();
        println!("{:?}", encoding);
        let encoding = encoder.encode_char();
        println!("{:?}", encoding);
        // assert_eq!(encoding, vec![1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0]);
        // let encoding = encoder.encode_char();
        // assert_eq!(encoding, vec![0, 1, 0, 1, 1, 0, 1, 0]);
        // let encoding = encoder.encode_char();
        // assert_eq!(encoding, vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
        // println!("{:?}", encoding);
        // let encoding = encoder.encode_char();
        // assert_eq!(encoding, vec![]);
        // assert_eq!(encoding, vec![]);
    }
}
