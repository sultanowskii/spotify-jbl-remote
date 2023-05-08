use zbus::dbus_proxy;
use zbus::blocking::Connection;

use crate::input_event::{InputEvent, EventType, EventCode};

// Spotify DBus proxy (zbus).
#[dbus_proxy(
    // default_service = "org.mpris.MediaPlayer2.firefox.instance728",
    default_service = "org.mpris.MediaPlayer2.spotify",
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait SpotifyPlayer {
    fn play(&self) -> zbus::Result<()>;
    fn pause(&self) -> zbus::Result<()>;
    fn playpause(&self) -> zbus::Result<()>;
    fn next(&self) -> zbus::Result<()>;
    fn previous(&self) -> zbus::Result<()>;
}

// Spotify DBus communicator.
#[derive(Debug)]
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

    // "Play/Pause" command
    pub fn playpause(&self) -> zbus::Result<()> {
        self.proxy.playpause()?;
        Ok(())
    }

    // "Next track" command
    pub fn next(&self) -> zbus::Result<()> {
        self.proxy.next()?;
        Ok(())
    }

    // "Previous track" command
    pub fn previous(&self) -> zbus::Result<()> {
        self.proxy.previous()?;
        Ok(())
    }
}
