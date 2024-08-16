extern crate rodio;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{self};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use console::{Term, style};
use crossterm::{event::{self, Event, KeyCode}, terminal::{enable_raw_mode}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the audio file
    let song_title = "Journey(chosic.com).mp3";
    let file = File::open(song_title)?;
    let reader = io::BufReader::new(file);
    let source = Decoder::new(reader).unwrap();

    // Create a message channel to communicate with the main thread
    let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();

    // Get the output stream
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    // Play the audio in a separate thread
    std::thread::spawn(move || {
        sink.append(source);
        // sink.play();

        loop {
            match rx.recv() {
                Ok('p') => {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                Ok('q') | Err(_) => break,
                _ => {}
            }

            if sink.empty() {
                break;
            }
        }

        sink.sleep_until_end();
    });

    let term = Term::stdout();

    // Enable raw mode for keyboard input
    enable_raw_mode()?;

    loop {
        // Poll for keyboard events
        if let Event::Key(key) = event::read().unwrap() {
            // Check if the user pressed ESC
            if key.code == KeyCode::Char('q') {
                println!("Keyboard Interrupt");
                break;
            }
            if key.code == KeyCode::Char('p') {

                if event::poll(Duration::from_millis(100)).unwrap() == true {
                    continue;
                }
                tx.send('p').unwrap();

            }
            if key.code == KeyCode::Esc {
                println!("Keyboard Interrupt");
                break;
            }
        }

        // Clear the previous line and print the new progress
        print!("\x1Bc");
        term.clear_line()
            .unwrap();
        term.write_line(
            &format!("Playing: {}%", style(song_title)),
        )
            .unwrap();

        sleep(Duration::from_millis(500)); // Update every 500ms for smoother progress

        // Check if the audio has finished playing
    }

    Ok(())
}