use std::io;
use std::io::Write;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::task;
use crate::song_option::SongOptions;
mod scrapper;
mod cache_manager;
mod model;
mod string_formatter;
mod song_option;

#[tokio::main]
async fn main() {
    let songs = cache_manager::load_songs().await;
    let mut song_options = SongOptions::new(songs);

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
                        KeyCode::Up => {
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.move_selection(true);
                                continue;
                            }
                        }
                        KeyCode::Down => {
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.move_selection(false);
                                continue;
                            }
                        }
                        KeyCode::Char('r') => {
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.select_delete();
                                continue;
                            }
                        }
                        KeyCode::Char(c) => {
                            if event::poll(Duration::from_millis(500)).unwrap() {
                                write!(stdout, "\x1Bc").unwrap();
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
            song_options.print();
        }
        disable_raw_mode().unwrap();
    }).await.unwrap();
}
