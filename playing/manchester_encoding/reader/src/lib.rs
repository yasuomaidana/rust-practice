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
}
