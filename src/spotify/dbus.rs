use zbus::dbus_proxy;
use zbus::blocking::Connection;

// Spotify DBus proxy (zbus).
#[dbus_proxy(
    default_service = "org.mpris.MediaPlayer2.spotify",
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait SpotifyPlayer {
    fn Play(&self) -> zbus::Result<()>;
    fn Pause(&self) -> zbus::Result<()>;
    fn PlayPause(&self) -> zbus::Result<()>;
    fn Next(&self) -> zbus::Result<()>;
    fn Previous(&self) -> zbus::Result<()>;
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
        self.proxy.Play()?;
        Ok(())
    }

    // "Pause" command
    pub fn pause(&self) -> zbus::Result<()> {
        self.proxy.Pause()?;
        Ok(())
    }

    // "Play/Pause" command
    pub fn playpause(&self) -> zbus::Result<()> {
        self.proxy.PlayPause()?;
        Ok(())
    }

    // "Next track" command
    pub fn next(&self) -> zbus::Result<()> {
        self.proxy.Next()?;
        Ok(())
    }

    // "Previous track" command
    pub fn previous(&self) -> zbus::Result<()> {
        self.proxy.Previous()?;
        Ok(())
    }
}
