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
    format!("\x1b[32m{}\x1b[0m", text)
}