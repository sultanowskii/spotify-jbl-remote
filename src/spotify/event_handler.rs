use crate::input_event::{EventType, EventCode, InputEvent};

use super::SpotifyDBus;

#[derive(Debug)]
pub struct EventHandler<'a> {
    pub dbus: SpotifyDBus<'a>,
}

impl<'a> EventHandler<'a> {
    // Perform an action based on input event.
    pub fn handle_input_event(&self, input_event: &InputEvent) -> zbus::Result<()> {
        // we want button to be unpressed + ignoring syn events
        if input_event.type_ == EventType::Syn || input_event.value != 0 {
            return Ok(());
        }

        match input_event.type_ {
            EventType::Key => {},
            EventType::Syn => return Ok(()),
            EventType::Unused => return Ok(()),
        };

        match input_event.code {
            EventCode::PlayCD => self.dbus.play(),
            EventCode::PauseCD => self.dbus.pause(),
            EventCode::NextSong => self.dbus.next(),
            EventCode::Default => Ok(()),
            EventCode::Unused => return Ok(()),
        }?;

        println!("{}", input_event);

        Ok(())
    }
}