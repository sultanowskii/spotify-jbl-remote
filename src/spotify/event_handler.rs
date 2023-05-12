use crate::input_event::{EventType, InputEvent};

use super::{SpotifyDBus, action::Action, event_handle_driver::EventHandleDriver};

// Event handler
#[derive(Debug)]
pub struct EventHandler<'a> {
    pub dbus: SpotifyDBus<'a>,
}

impl<'a> EventHandler<'a> {
    // Perform an action based on input event.
    pub fn handle_input_event<T: EventHandleDriver + ?Sized>(&self, driver: &T, input_event: &InputEvent) -> zbus::Result<()> {
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
            Action::PreviousSong => self.dbus.previous(),
            Action::Other(f) => Ok(f(&event_code)),
            Action::Nothing => return Ok(()),
        }?;

        println!("{} -> {}", input_event, action);

        Ok(())
    }
}
