
use zbus::{Result, dbus_proxy, zvariant::Value};

#[dbus_proxy(
    default_service = "org.mpris.MediaPlayer2.spotify",
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait SpotifyMediaPlayer {
    #[dbus_proxy(name = "Next")]
    async fn next(&self) -> Result<()>;

    #[dbus_proxy(name = "PlayPause")]
    async fn play_pause(&self) -> Result<()>;

    #[dbus_proxy(name = "Previous")]
    async fn previous(&self) -> Result<()>;

    #[dbus_proxy(property, name = "CanGoNext")]
    fn can_next(&self) -> Result<bool>;

    #[dbus_proxy(property, name = "CanGoPrevious")]
    fn can_prev(&self) -> Result<bool>;

    #[dbus_proxy(property, name = "Position")]
    fn position(&self) -> Result<i64>;

    #[dbus_proxy(property, name = "LoopStatus")]
    fn _loop_status(&self) -> Result<String>;

    #[dbus_proxy(property, name = "LoopStatus")]
    fn _set_loop_status(&self, value: String) -> Result<()>;

    #[dbus_proxy(property, name = "PlaybackStatus")]
    fn _playback_status(&self) -> Result<String>;

    #[dbus_proxy(property, name = "PlaybackStatus")]
    fn _set_playback_status(&self, value: String) -> Result<()>;

    #[dbus_proxy(property, name = "Metadata")]
    fn metadata(&self) -> Result<&Metadata>;
}

