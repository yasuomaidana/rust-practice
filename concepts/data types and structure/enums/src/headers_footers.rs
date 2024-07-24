fn string_repeat(n: i32, c: char)-> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push(c);
    }
    return s;
}

fn enclose_message(message: &str, enclose: String) -> String {
    let mut s = String::new();
    for char_ in enclose.chars() {
        s.push(char_);
    }
    s.push_str(message);
    for char_ in enclose.chars() {
        s.push(char_);
    }
    return s;

}
pub(crate) fn print_header(message: &str, width: i32) {
    let remaining = width - message.len() as i32;
    let half = remaining / 2 -1;
    let filler = string_repeat(half, '=');
    let message = enclose_message(enclose_message(message, filler).as_str(),
                                  "|".to_string());
    println!("{}", message);
}


pub(crate) fn print_sub_header(message: &str, width: i32) {
    let remaining = width - message.len() as i32;
    let half = remaining / 2 - 2;
    let filler = string_repeat(half, '.');
    let mut message = enclose_message(enclose_message(message, filler).as_str(),
                                      ";".to_string());

    message = enclose_message(message.as_str(), "_".to_string());
    println!("{}", message);
}

pub(crate) fn print_footer(width: i32) {
    let message_length = width - 2;
    let message = enclose_message(string_repeat(message_length, '-').as_str(),
                                  ":".to_string());
    println!("{}", message);
}