mod loop_status;
pub mod playback_status;
mod metadata;

use zbus::{dbus_proxy, Result, Connection, CacheProperties};

use self::{metadata::TrackMetadata, playback_status::PlaybackStatus, loop_status::LoopStatus};

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

    #[dbus_proxy(name = "Seek")]
    async fn seek_plus_one_second(&self, pos: i64) -> Result<()>;

    #[dbus_proxy(property, name = "CanGoNext")]
    fn can_next(&self) -> Result<bool>;

    #[dbus_proxy(property, name = "CanGoPrevious")]
    fn can_prev(&self) -> Result<bool>;

    #[dbus_proxy(property, name = "Position")]
    fn position(&self) -> Result<i64>;

    #[dbus_proxy(property, name = "LoopStatus")]
    fn loop_status(&self) -> Result<LoopStatus>;

    #[dbus_proxy(property, name = "LoopStatus")]
    fn set_loop_status(&self, value: LoopStatus) -> Result<()>;

    #[dbus_proxy(property, name = "PlaybackStatus")]
    fn playback_status(&self) -> Result<PlaybackStatus>;

    #[dbus_proxy(property, name = "PlaybackStatus")]
    fn _set_playback_status(&self, value: PlaybackStatus) -> Result<()>;

    #[dbus_proxy(property, name = "Metadata")]
    fn metadata(&self) -> Result<TrackMetadata>;
}

pub async fn spotify_proxy() -> Result<SpotifyMediaPlayerProxy<'static>> {
    SpotifyMediaPlayerProxy::builder(&Connection::session().await?)
    .cache_properties(CacheProperties::No)
    .build().await
}

