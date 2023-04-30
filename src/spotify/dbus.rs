use zbus::dbus_proxy;
use zbus::blocking::Connection;

use crate::input_event::{InputEvent, EventType, EventCode};

// Spotify DBus proxy (zbus).
#[dbus_proxy(
    default_service = "org.mpris.MediaPlayer2.spotify",
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait SpotifyPlayer {
    fn play(&self) -> zbus::Result<String>;
    fn pause(&self) -> zbus::Result<String>;
    fn next(&self) -> zbus::Result<String>;
}

// Spotify DBus communicator.
pub struct SpotifyDBus<'a> {
    proxy: SpotifyPlayerProxyBlocking<'a>,
}

impl<'a> SpotifyDBus<'a> {
    // Create new instance (and connect to Spotify DBus).
    pub fn new() -> zbus::Result<Self> {
        let connection = Connection::session()?;
        let proxy = SpotifyPlayerProxyBlocking::new(&connection).unwrap();
        Ok(
            SpotifyDBus {
                proxy: proxy,
            }
        )
    }

    // "Play" command
    pub fn play(&self) -> zbus::Result<()> {
        self.proxy.play()?;
        Ok(())
    }

    // "Pause" command
    pub fn pause(&self) -> zbus::Result<()> {
        self.proxy.pause()?;
        Ok(())
    }

    // "Next" command
    pub fn next(&self) -> zbus::Result<()> {
        self.proxy.next()?;
        Ok(())
    }

    // Perform an action based on input event.
    pub fn handle_input_event(&self, input_event: InputEvent) -> zbus::Result<()> {
        // we want button to be unpressed + ignoring syn events
        if input_event.type_ == EventType::Syn || input_event.value != 0 {
            return Ok(());
        }

        match input_event.code {
            EventCode::PlayCD => self.play(),
            EventCode::PauseCD => self.pause(),
            EventCode::NextSong => self.next(),
            EventCode::Default => Ok(()),
        }?;

        Ok(())
    }
}
