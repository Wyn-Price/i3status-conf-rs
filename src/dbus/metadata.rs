
use zbus::{zvariant::{DeserializeDict, SerializeDict, Type, Value, OwnedValue}};

#[derive(DeserializeDict, SerializeDict, Type, Debug, Value, OwnedValue, PartialEq, Eq)]
#[zvariant(signature = "a{sv}")]
pub struct TrackMetadata {
    #[zvariant(rename = "mpris:trackid")]
    track_id: String,

    #[zvariant(rename = "mpris:length")]
    track_length: String,

    #[zvariant(rename = "mpris:artUrl")]
    art_url: String,

    #[zvariant(rename = "xesam:album")]
    album: String,

    #[zvariant(rename = "xesam:albumArtist")]
    album_artist: Vec<String>,

    #[zvariant(rename = "xesam:artst")]
    artist: Vec<String>,

    #[zvariant(rename = "xesam:discNumber")]
    disc_number: i32,

    #[zvariant(rename = "xesam:title")]
    title: String,

    #[zvariant(rename = "xesam:trackNumber")]
    track_number: i32,

    #[zvariant(rename = "xesam:url")]
    url: String,
}
