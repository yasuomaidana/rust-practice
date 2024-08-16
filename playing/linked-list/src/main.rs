extern crate rodio;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{self};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use console::{Term, style};
use crossterm::{event::{self, Event, KeyCode}, terminal::{ enable_raw_mode}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the audio file
    let song_title = "Journey(chosic.com).mp3";
    let file = File::open(song_title)?;
    let reader = io::BufReader::new(file);
    let source = Decoder::new(reader).unwrap();

    // Create a message channel to communicate with the main thread
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    // Get the output stream
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    // Play the audio in a separate thread
    std::thread::spawn(move || {
        sink.append(source);
        sink.play();

        // Wait for the audio to finish playing before sending the signal
        sink.sleep_until_end();
        tx.send(()).unwrap(); // Signal the main thread that playback is finished




    });

    let term = Term::stdout();

    // Enable raw mode for keyboard input
    enable_raw_mode()?;

    loop {
        // Poll for keyboard events
        if let Event::Key(key) = event::read()? {
            // Check if the user pressed ESC
            if key.code == KeyCode::Char('q') {
                println!("Keyboard Interrupt");
                break;
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
        if rx.try_recv().is_ok() {
            break;
        }
    }

    Ok(())
}