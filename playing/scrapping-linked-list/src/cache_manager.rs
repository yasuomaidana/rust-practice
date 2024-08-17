use std::fs::File;
use std::io::Read;
use chrono::{Datelike, Local};
use crate::model::{CachedSong, Song};

pub async fn load_songs() -> Vec<Song> {
    match _load_songs() {
        Some(songs) => songs,
        None => {
            let songs = crate::scrapper::load_songs().await;
            let cache = CachedSong {
                songs,
                last_update: Local::now().day(),
            };
            let cache_memory = File::create("cache_memory.json").expect("Failed to create cache file");
            serde_json::to_writer(cache_memory, &cache).expect("Failed to write to cache file");
            cache.songs
        }
    }
}

fn _load_songs() -> Option<Vec<Song>> {
    let cache_memory = File::open("cache_memory.json");
    match cache_memory {
        Ok(file) => {
            let mut cache_memory = file;
            let mut cache = String::new();
            cache_memory.read_to_string(&mut cache).expect("Failed to read cache file");
            let cache: CachedSong = serde_json::from_str(&cache).expect("Failed to parse cache file");
            if cache.last_update == Local::now().day() {
                Some(cache.songs)
            } else {
                None
            }
        },
        Err(_) => None
    }

}