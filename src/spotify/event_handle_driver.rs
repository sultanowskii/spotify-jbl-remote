use crate::input_event::EventCode;

use super::action::Action;

pub trait EventHandleDriver {
    fn get_action_from_event(&self, event_code: &EventCode) -> Action;
}

// Play-Pause event handle driver:
// play == play()
// pause == pause()
// next == next()
// other event codes are ignored
#[derive(Debug)]
pub struct DefaultEventHandleDriver {}

impl EventHandleDriver for DefaultEventHandleDriver {
    fn get_action_from_event(&self, event_code: &EventCode) -> Action {
        match event_code {
            EventCode::PlayCD => Action::Play,
            EventCode::PauseCD => Action::Pause,
            EventCode::NextSong => Action::NextSong,
            EventCode::Default => Action::Nothing,
            EventCode::Unused(_) => Action::Nothing,
        }
    }
}

// Play-Pause event handle driver:
// play == playpause()
// pause == playpause()
// next == next()
// other event codes are ignored
#[derive(Debug)]
pub struct PlayPauseEventHandleDriver {}

impl EventHandleDriver for PlayPauseEventHandleDriver {
    fn get_action_from_event(&self, event_code: &EventCode) -> Action {
        match event_code {
            EventCode::PlayCD => Action::PlayPause,
            EventCode::PauseCD => Action::PlayPause,
            EventCode::NextSong => Action::NextSong,
            EventCode::Default => Action::Nothing,
            EventCode::Unused(_) => Action::Nothing,
        }
    }
}

// Debug event handle driver
#[derive(Debug)]
pub struct VoidEventHandleDriver {}

impl EventHandleDriver for VoidEventHandleDriver {
    fn get_action_from_event(&self, _event_code: &EventCode) -> Action {
        Action::Other(|event_code| -> () {println!("{}", event_code)})
    }
}


#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DriverName {
    Default,
    PlayPause,
    Void,
}

impl DriverName {
    pub fn try_into_driver(&self) -> Box<dyn EventHandleDriver> {
        match self {
            DriverName::Default => Box::new(DefaultEventHandleDriver {}),
            DriverName::PlayPause => Box::new(PlayPauseEventHandleDriver {}),
            DriverName::Void => Box::new(VoidEventHandleDriver {}),
        }
    }
}