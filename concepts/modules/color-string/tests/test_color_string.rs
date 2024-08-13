use color_string::color_string::ColorString;
use color_string::color_text::Color;
#[test]
fn test_resetting_string_color() {
    let text = "Hello, world!";
    let mut color_string = ColorString::new(Some(Color::Red), text);
    // println!("{}", color_string.colorized);
    // color_string.reset();
    //
    // println!("{}", color_string.colorized);
    // assert_eq!(color_string.colorized, text);
}

#[test]
fn test_changing_string_color() {
    let text = "Hello, world!";
    let mut color_string = ColorString::new(Some(Color::Red), text);
    color_string.change_color(Color::Green);
    assert_eq!(color_string.colorized, "\x1b[32mHello, world!\x1b[0m");
}