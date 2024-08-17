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
}
