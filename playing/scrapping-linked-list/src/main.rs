use std::io;
use std::io::Write;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::task;
use crate::string_formatter::{color_string, Color};
mod scrapper;
mod cache_manager;
mod model;
mod string_formatter;

#[tokio::main]
async fn main() {
    let songs = cache_manager::load_songs().await;
    let mut i = 0;
    for song in songs {
        if i == 0 {
            let content = format!("{}-{}", song.title, song.artist);
            println!("{}", color_string(&content, Some(Color::Red)));
        } else if i == 2 {
            let content = format!("{}-{}", song.title, song.artist);
            println!("{}", color_string(&content, Some(Color::Green)));
        } else {
            println!("{}-{}", song.title, song.artist);
        }
        i += 1;
    }


    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    writeln!(stdout, "Press 'q' to exit.").unwrap();
    stdout.flush().unwrap();

    task::spawn(async move {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            writeln!(stdout, "Exiting...").unwrap();
                            break;
                        }
                        KeyCode::Char(c) => {
                            if event::poll(Duration::from_millis(500)).unwrap(){
                                writeln!(stdout, "You pressed: {}", c).unwrap();
                                continue;
                            }
                        }
                        KeyCode::Esc => {
                            writeln!(stdout, "Exiting...").unwrap();
                            break;
                        }
                        _ => {}
                    }
                    stdout.flush().unwrap();
                }
            }
        }
        disable_raw_mode().unwrap();
    }).await.unwrap();


}
