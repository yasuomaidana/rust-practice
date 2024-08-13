/// Formats the given text with ANSI escape codes to display it in green color.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be formatted.
///
/// # Returns
///
/// A `String` containing the original text wrapped in ANSI escape codes for green color.
///
/// # Examples
///
/// ```
/// use file_reader::color_text::green;
/// let green_text = green("Hello, world!");
/// println!("{}", green_text); // This will print "Hello, world!" in green color.
/// ```
pub fn green(text: &str) -> String {
    formatter(Color::Green, text)
}

pub fn red(text: &str) -> String {
    formatter(Color::Red, text)
}

pub fn yellow(text: &str) -> String {
    formatter(Color::Yellow, text)
}

pub fn blue(text: &str) -> String {
    formatter(Color::Blue, text)
}

pub fn magenta(text: &str) -> String {
    formatter(Color::Magenta, text)
}

pub fn cyan(text: &str) -> String {
    formatter(Color::Cyan, text)
}

pub fn white(text: &str) -> String {
    formatter(Color::White, text)
}

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

fn formatter(color: Color, text: &str) -> String {
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

pub fn reset(text: &str) -> String {
    formatter(Color::Default, text)
}