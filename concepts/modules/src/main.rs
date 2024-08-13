
use color_string::color_text::Color::{Blue, Cyan, Green, Magenta, Red, White, Yellow};
use color_string::color_text::formatter;

fn main() {
    let file = file_reader::TextFile::read_from_console();
    let lines = file.get_lines();
    let colors = vec![
        Green,
        Red,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White
    ];
    for (i, line) in lines.iter().enumerate() {
        let color = &colors[i % colors.len()];
        println!("{}",formatter(color, line));
    }
    println!("Read file {}", formatter(&Green, file.get_file_name()));
}
