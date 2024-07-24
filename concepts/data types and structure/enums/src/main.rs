// Plain enum
// enum WebEvents{
//     WELoad,
//     WEKeys(String, char),
//     WebClick{x: i64, y: i64, button: i64},
// }

use crate::headers_footers::{print_footer, print_header, print_sub_header};

mod headers_footers;
#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
    button: i64,
}

#[derive(Debug)]
enum WebEvents {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

trait PrintEvent {
    fn print_event(&self);
}

impl PrintEvent for WebEvents {
    fn print_event(&self) {
        match self {
            WebEvents::WELoad( x) => println!("Tuple Web Load Event: {}", x),
            WebEvents::WEClick( mouse) =>
            match  { mouse } {
                MouseClick{button, .. } if *button == 1 =>{
                    println!("Web Click Event: left button");
                }
                MouseClick{button, .. } if *button == 0 =>{
                    println!("Web Click Event: Right button");
                }
                MouseClick { x, y, button } => {
                    println!("Web Click Event: x: {}, y: {}, button: {}", x, y, button);
                }
            },
            WebEvents::WEKeys(key) =>
            match key {
                KeyPress(key, value) => {
                    println!("Web Keys Event: key: {}, value: {}", key, value);
                }
            }
        }
    }
}

fn main() {
    let message_length = 40;
    print_header("Defining enums", message_length);
    let we_load = WebEvents::WELoad(true);
    let we_click = WebEvents::WEClick(MouseClick {
        x: 10,
        y: 20,
        button: 1,
    });
    let we_keys = WebEvents::WEKeys(KeyPress("Ctrl+".to_string(), 'z'));
    print_sub_header("Simple printing", message_length);
    println!("Web Load Event {:#?}", we_load);
    println!("Web Click Event {:#?}", we_click);
    println!("Web Keys Event {:#?}", we_keys);
    print_sub_header("Simple printing: Matching pattern", message_length);
    print!("Web Load Event: unnamed struct");
    PrintEvent::print_event(&we_load);
    PrintEvent::print_event(&we_click);
    PrintEvent::print_event(&we_keys);

    print_footer(message_length);
}
