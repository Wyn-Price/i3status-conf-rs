// Currently, the OwnedValue marco does not support renaming,
// meaning for now I'll just deserialise by hand...
// Tracked @ https://gitlab.freedesktop.org/dbus/zbus/-/issues/320

use std::collections::HashMap;

use zbus::zvariant::{Error, OwnedValue, Value};

// #[derive(SerializeDict, DeserializeDict, OwnedValue, Debug)]
// #[zvariant(signature = "a{sv}")]
#[derive(Debug)]
pub struct TrackMetadata {
    // #[zvariant(rename = "mpris:trackid")]
    pub track_id: String,

    // #[zvariant(rename = "mpris:length")]
    pub track_length: u64,

    // #[zvariant(rename = "mpris:artUrl")]
    pub art_url: String,

    // #[zvariant(rename = "xesam:album")]
    pub album: String,

    // #[zvariant(rename = "xesam:albumArtist")]
    pub album_artist: Vec<String>,

    // #[zvariant(rename = "xesam:artist")]
    pub artist: Vec<String>,

    // #[zvariant(rename = "xesam:discNumber")]
    pub disc_number: i32,

    // #[zvariant(rename = "xesam:title")]
    pub title: String,

    // #[zvariant(rename = "xesam:trackNumber")]
    pub track_number: i32,

    // #[zvariant(rename = "xesam:url")]
    pub url: String,
}

macro_rules! pull_from_map {
    ($map:expr, $str:expr) => {
        {
            let r = $map.remove($str).ok_or_else(|| Error::Message(format!("Entry for '{}' not found", $str)))?;
            let t = format!("{:#?}", &r);
            r.downcast().ok_or_else(|| Error::Message(format!("Invalid type for '{}', found {}", $str, t)))?
        }
    };
}
impl TryFrom<OwnedValue> for TrackMetadata {
    type Error = Error;
    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let mut map: HashMap<String, Value> = value.try_into()?;
        return Ok(TrackMetadata {
            track_id: pull_from_map!(map, "mpris:trackid"),
            track_length: pull_from_map!(map, "mpris:length"),
            art_url: pull_from_map!(map, "mpris:artUrl"),
            album: pull_from_map!(map, "xesam:album"),
            album_artist: pull_from_map!(map, "xesam:albumArtist"),
            artist: pull_from_map!(map, "xesam:artist"),
            disc_number: pull_from_map!(map, "xesam:discNumber"),
            title: pull_from_map!(map, "xesam:title"),
            track_number: pull_from_map!(map, "xesam:trackNumber"),
            url: pull_from_map!(map, "xesam:url"),
        });
    }
}

