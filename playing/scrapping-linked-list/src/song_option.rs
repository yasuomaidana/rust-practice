use std::collections::LinkedList;
use std::io::{self, Write};
use crate::model::Song;
use crate::song_option::SongOptionState::Unselected;

enum SongOptionState{
    Selected,
    Unselected,
    Hovered,
}

struct SongOption{
    song: Song,
    selected: SongOptionState,
}

impl SongOption {
    fn new(song: Song) -> Self {
        Self {
            song,
            selected: Unselected,
        }
    }
}

pub struct SongOptions {
    options: LinkedList<SongOption>,
    selected_index: usize,
}

impl SongOptions {
    pub fn new(songs: Vec<Song>) -> Self {
        let mut options = LinkedList::new();
        for song in songs {
            options.push_back(SongOption::new(song));
        }
        Self {
            options,
            selected_index: 0,
        }
    }

    pub fn print(&self) {
        let mut i = 0;
        let mut stdout = io::stdout();
        write!(stdout, "\x1Bc").unwrap();
        for option in &self.options {
            if i == self.selected_index {
                writeln!(stdout, "> {}-{}", option.song.title, option.song.artist).unwrap();
            } else {
                writeln!(stdout,"  {}-{}", option.song.title, option.song.artist).unwrap();
            }
            i += 1;
        }
    }
}