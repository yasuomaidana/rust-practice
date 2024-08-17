mod scrapper;
mod cache_manager;
mod model;

#[tokio::main]
async fn main() {

    let songs = cache_manager::load_songs().await;
    for song in songs {
        println!("{}-{}", song.title, song.artist);
    }
}
