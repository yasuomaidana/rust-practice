

fn main() {
    let file = file_reader::TextFile::read_from_console();
    let lines = file.get_lines();
    for line in lines {
        println!("{}", line);
    }
    println!("Read file {}", file.get_file_name());
}
