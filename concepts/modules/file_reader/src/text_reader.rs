use std::io;
use std::io::{BufRead, BufReader};


/// Reads the contents of a file and returns its lines as a string.
///
/// # Returns
/// A `String` containing the contents of the file.
///
///  #Examples :
///
/// ```
/// use file_reader::text_reader::read_stdin;
/// let input = read_stdin();
/// println!("{}", input);
/// ```
pub fn read_stdin() -> String {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_stdin() {
        let input = "Hello, world!\n";
        let cursor = Cursor::new(input);
        let mut reader = BufReader::new(cursor);
        assert_eq!(_read_stdin(&mut reader), input.trim(), "Failed to read from stdin");
    }
}