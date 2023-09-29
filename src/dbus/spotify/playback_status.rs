use zbus::zvariant::{Error, OwnedValue, Value};

#[derive(Debug)]
pub enum PlaybackStatus {
    Playing, Paused
}

impl TryFrom<OwnedValue> for PlaybackStatus {
    type Error = Error;
    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let str: &str = value.downcast_ref().ok_or(Error::Message(String::from("Could not convert value to a string")))?;
        match str {
            "Playing" => Ok(PlaybackStatus::Playing),
            "Paused" => Ok(PlaybackStatus::Paused),
            _ => Err(Error::Message(format!("Unknown playback status {str}"))),
        }
    }
}

impl From<PlaybackStatus> for Value<'_> {
    fn from(value: PlaybackStatus) -> Self {
        let str = match value {
            PlaybackStatus::Playing => "Playing",
            PlaybackStatus::Paused => "Paused",
        };
        return Value::Str(str.into());
    }
}