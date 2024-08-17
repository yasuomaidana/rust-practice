pub enum Color {
    Red,
    Green,
    // Blue,
}
pub fn color_string(s: &str, color: Option<Color>) -> String {
    let color = match color {
        Some(Color::Red) => "\x1b[31m",
        Some(Color::Green) => "\x1b[32m",
        // Some(Color::Blue) => "\x1b[34m",
        None => ""
    };
    format!("{}{}\x1b[0m", color, s)
}