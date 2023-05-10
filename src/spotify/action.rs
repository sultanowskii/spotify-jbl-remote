use crate::input_event::EventCode;

pub enum Action {
    Play,
    Pause,
    PlayPause,
    NextSong,
    Other(fn(&EventCode) -> ()),
    Nothing,
}