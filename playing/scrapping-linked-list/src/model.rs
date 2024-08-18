use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Song{
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSong {
    pub songs: Vec<Song>,
    pub last_update: u32,
}