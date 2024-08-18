use std::collections::LinkedList;
use std::io::{self, Write};
use crate::model::Song;
use crate::song_option::SongOptionState::Unselected;
use crate::string_formatter::{bold_string, color_string, Color};

enum SongOptionState {
    Selected,
    Unselected,
    ToDelete
}

struct SongOption {
    song: Song,
    selected: SongOptionState,
    hovered: bool,
}

impl SongOption {
    fn new(song: Song) -> Self {
        Self {
            song,
            selected: Unselected,
            hovered: false,
        }
    }
    fn format(&self) -> String {
        let selected = match self.selected {
            SongOptionState::Selected => format!("> {}-{}", self.song.title, self.song.artist),
            _ => format!("  {}-{}", self.song.title, self.song.artist),
        };
        match self.hovered {
            true => bold_string(&selected),
            false => selected,
        }
    }
    fn color_format(&self) -> String {
        match self.selected {
            SongOptionState::Selected => color_string(&self.format(), Some(Color::Green)),
            SongOptionState::ToDelete => color_string(&self.format(), Some(Color::Red)),
            Unselected => self.format(),
        }.to_string()
    }
}

pub struct SongOptions {
    options: LinkedList<SongOption>,
}


impl SongOptions {
    pub fn new(songs: Vec<Song>) -> Self {
        let mut options = LinkedList::new();
        for song in songs {
            options.push_back(SongOption::new(Song::from(song)));
        }
        if let Some(first) = options.front_mut() {
            first.selected = SongOptionState::Selected;
        }
        Self {
            options
        }
    }

    pub fn print(&self) {
        let mut stdout = io::stdout();
        write!(stdout, "\x1Bc").unwrap();
        for option in &self.options {
            writeln!(stdout, "{}", option.color_format()).unwrap();
        }
    }
}