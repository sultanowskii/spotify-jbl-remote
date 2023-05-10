use crate::input_event::{EventCode, EventType, InputEvent};

use super::{SpotifyDBus, action::Action};

// Event handler
#[derive(Debug)]
pub struct EventHandler<'a> {
    pub dbus: SpotifyDBus<'a>,
}

impl<'a> EventHandler<'a> {
    // Perform an action based on input event.
    pub fn handle_input_event<T>(&self, driver: &T, input_event: &InputEvent) -> zbus::Result<()>
    where
        T: EventHandleDriver,
    {
        let InputEvent { type_: event_type, code: event_code, value: event_value, .. } = input_event;
        
        // we want button to be unpressed + ignoring syn events
        if *event_type == EventType::Syn || *event_value != 0 {
            return Ok(());
        }

        match event_type {
            EventType::Key => {}
            EventType::Syn => return Ok(()),
            EventType::Unused => return Ok(()),
        };

        let action = driver.get_action_from_event(&event_code);

        match action {
            Action::Play => self.dbus.play(),
            Action::Pause => self.dbus.pause(),
            Action::PlayPause => self.dbus.playpause(),
            Action::NextSong => self.dbus.next(),
            Action::Other(f) => Ok(f(&event_code)),
            Action::Nothing => return Ok(()),
        }?;

        println!("{}", input_event);

        Ok(())
    }
}

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
pub struct DebugEventHandleDriver {}

impl EventHandleDriver for DebugEventHandleDriver {
    fn get_action_from_event(&self, _event_code: &EventCode) -> Action {
        Action::Other(|event_code| -> () {println!("{}", event_code)})
    }
}
