use std::collections::LinkedList;
use std::io::{self, Write};
use std::path::Iter;
use std::slice::IterMut;
use crate::model::Song;
use crate::song_option::SongOptionState::{Selected, Unselected};
use crate::string_formatter::{bold_string, color_string, Color};

#[derive(PartialEq)]
enum SongOptionState {
    Selected,
    Unselected,
    ToDelete,
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
            options,
        }
    }

    pub fn print(&self) {
        let mut stdout = io::stdout();
        write!(stdout, "\x1Bc").unwrap();
        for option in &self.options {
            writeln!(stdout, "{}", option.color_format()).unwrap();
        }
    }

    pub fn move_selection(&mut self, up: bool) {
        let len = self.options.len();
        if len == 0 {
            return;
        }
        let mut iterator = self.options.iter_mut();
        if up {
            while let Some(current) = iterator.next_back() {
                if current.selected == Selected {
                    current.selected = Unselected;
                    let next = iterator.next_back();
                    match next {
                        Some(next) => { next.selected = Selected;
                            return;
                        },
                        None => {
                            let last = self.options.back_mut().unwrap();
                            last.selected = Selected;
                            return;
                        },
                    }

                }
            }
        } else {
            let current = iterator.next().unwrap();
            while let Some(next) = iterator.next() {
                if current.selected == Selected {
                    current.selected = Unselected;
                    next.selected = Selected;
                    return;
                }
            }
        }
    }
}