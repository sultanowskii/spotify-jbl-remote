use std::fmt::Display;

use crate::input_event::EventCode;

pub enum Action {
    Play,
    Pause,
    PlayPause,
    NextSong,
    PreviousSong,
    Other(fn(&EventCode) -> ()),
    Nothing,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Play => write!(f, "Play"),
            Action::Pause => write!(f, "Pause"),
            Action::PlayPause => write!(f, "PlayPause"),
            Action::NextSong => write!(f, "NextSong"),
            Action::PreviousSong => write!(f, "PreviousSong"),
            Action::Other(_) => write!(f, "Other(?)"),
            Action::Nothing => write!(f, "Nothing"),
        }
        
    }
}