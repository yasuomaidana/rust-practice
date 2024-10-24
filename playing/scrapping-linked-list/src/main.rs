use std::io;
use std::io::Write;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{ Event, KeyCode};
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
            if event::poll(Duration::from_millis(500)).unwrap() {
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
                        KeyCode::Enter =>{
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.delete_selected();
                                continue;
                            }
                        }

                        KeyCode::Char('u') => {
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.move_selected(true);
                                continue;
                            }
                        }
                        KeyCode::Char('d') => {
                            if event::poll(Duration::from_millis(100)).unwrap() {
                                song_options.move_selected(false);
                                continue;
                            }
                        }
                        KeyCode::Esc => {
                            writeln!(stdout, "Exiting...").unwrap();
                            break;
                        }
                        _ => {
                            song_options.print();
                        }
                    }
                }
            }
            song_options.print();
            println!("Press 'q' to exit.");
            stdout.flush().unwrap();
        }
        disable_raw_mode().unwrap();
    }).await.unwrap();
}
