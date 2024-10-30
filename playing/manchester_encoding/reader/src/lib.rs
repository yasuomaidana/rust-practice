
mod reader;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub(crate) fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

pub(crate) fn delete_file<P>(filename: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    std::fs::remove_file(filename)
}

fn char_to_binary(c: char) -> String {
    format!("{:b}", c as u32)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn reading_text_file() {
        if Path::new("text.txt").exists() {
            delete_file("text.txt").expect("Unable to delete file");
        }
        fs::write("text.txt", "hello").expect("Unable to write file");
        let lines = read_lines("text.txt").expect("Unable to read file");
        assert_eq!(lines[0], "hello");
        delete_file("text.txt").unwrap()
    }

    #[test]
    fn char_to_binary_conversion() {
        assert_eq!(char_to_binary('a'), "1100001");
        assert_eq!(char_to_binary('b'), "1100010");
        assert_eq!(char_to_binary('c'), "1100011");
        assert_eq!(char_to_binary('d'), "1100100");
        assert_eq!(char_to_binary('e'), "1100101");
        assert_eq!(char_to_binary('f'), "1100110");
        assert_eq!(char_to_binary('g'), "1100111");
        assert_eq!(char_to_binary('h'), "1101000");
        assert_eq!(char_to_binary('i'), "1101001");
        assert_eq!(char_to_binary('j'), "1101010");
        assert_eq!(char_to_binary('k'), "1101011");
        assert_eq!(char_to_binary('l'), "1101100");
        assert_eq!(char_to_binary('m'), "1101101");
        assert_eq!(char_to_binary('n'), "1101110");
        assert_eq!(char_to_binary('o'), "1101111");
        assert_eq!(char_to_binary('p'), "1110000");
        assert_eq!(char_to_binary('q'), "1110001");
        assert_eq!(char_to_binary('r'), "1110010");
        assert_eq!(char_to_binary('s'), "1110011");
        assert_eq!(char_to_binary('t'), "1110100");
        assert_eq!(char_to_binary('u'), "1110101");
        assert_eq!(char_to_binary('v'), "1110110");
        assert_eq!(char_to_binary('w'), "1110111");
        assert_eq!(char_to_binary('x'), "1111000");
        assert_eq!(char_to_binary('y'), "1111001");
        assert_eq!(char_to_binary('z'), "1111010");

        println!("blank space{}",char_to_binary(' '));
        println!("new line{}",char_to_binary('\n'));
    }
}
