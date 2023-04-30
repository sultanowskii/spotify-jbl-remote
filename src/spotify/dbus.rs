use zbus::dbus_proxy;
use zbus::blocking::Connection;

#[dbus_proxy(
    default_service = "org.mpris.MediaPlayer2.spotify",
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait SpotifyPlayer {
    fn play(&self) -> zbus::Result<String>;
    fn pause(&self) -> zbus::Result<String>;
}

pub struct SpotifyDBus<'a> {
    proxy: SpotifyPlayerProxyBlocking<'a>,
}

impl<'a> SpotifyDBus<'a> {
    pub fn new() -> zbus::Result<Self> {
        let connection = Connection::session()?;
        let proxy = SpotifyPlayerProxyBlocking::new(&connection).unwrap();
        Ok(
            SpotifyDBus {
                proxy: proxy,
            }
        )
    }

    pub fn play(&self) -> zbus::Result<()> {
        self.proxy.play()?;
        Ok(())
    }


    pub fn pause(&self) -> zbus::Result<()> {
        self.proxy.pause()?;
        Ok(())
    }
}
