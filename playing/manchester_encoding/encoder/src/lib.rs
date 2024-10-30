use reader::char_to_binary;
use reader::reader::ReadData;


pub fn encode_char(c: char, previous_bit: u8) -> Vec<u8> {
    let binary = char_to_binary(c);
    let mut encoding = Vec::new();
    let mut previous_bit = previous_bit;
    for bite in binary.chars() {
        let bit = bite.to_digit(2).unwrap() as u8;
        if bit == 0 {
            encoding.push(1-previous_bit);
            encoding.push(previous_bit)
        }else{
            encoding.push(previous_bit);
            previous_bit = 1-previous_bit;
            encoding.push(previous_bit);

        }
    }

    encoding
}

pub fn generate_encoding(mut data_reader: impl ReadData) -> Vec<u8> {
    let mut encoding = Vec::new();
    while let Some(c) = data_reader.read_data() {
        encoding.push(c as u8);
    }
    encoding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_blank_space() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }

    #[test]
    fn test_encoding_blank_space() {
        let blank_encoded = encode_char(' ',1);
        assert_eq!(blank_encoded, vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0,1,0]);
    }


}
