use file_reader::color_text::{blue, cyan, green, magenta, red, white, yellow};

fn main() {
    let file = file_reader::TextFile::read_from_console();
    let lines = file.get_lines();
    let colors = vec![
        green,
        red,
        yellow,
        blue,
        magenta,
        cyan,
        white
    ];
    for (i, line) in lines.iter().enumerate() {
        let color = colors[i % colors.len()];
        println!("{}",color(line));
    }
    println!("Read file {}", green(file.get_file_name()));
}
