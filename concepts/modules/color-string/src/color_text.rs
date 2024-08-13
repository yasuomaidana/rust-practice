pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

pub fn formatter(color: &Color, text: &str) -> String {
    let code = match color {
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::Default => 0,
    };
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

// pub fn bold(text: &str) -> String {
//     format!("\x1b[1m{}\x1b[0m", text)
// }

pub fn reset(text: &str) -> String {
    formatter(&Color::Default, text)
}