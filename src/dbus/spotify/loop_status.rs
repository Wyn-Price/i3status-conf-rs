use zbus::zvariant::{Error, OwnedValue, Value};

#[derive(Debug)]
pub enum LoopStatus {
    // Loop forever
    Playlist,

    // Loop single
    Track,

    // Don't loop
    None
}

impl TryFrom<OwnedValue> for LoopStatus {
    type Error = Error;
    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let str: &str = value.downcast_ref().ok_or(Error::Message(String::from("Could not convert value to a string")))?;
        match str {
            "Playlist" => Ok(LoopStatus::Playlist),
            "Track" => Ok(LoopStatus::Track),
            "None" => Ok(LoopStatus::None),
            _ => Err(Error::Message(format!("Unknown status {str}"))),
        }
    }
}

impl From<LoopStatus> for Value<'_> {
    fn from(value: LoopStatus) -> Self {
        let str = match value {
            LoopStatus::Playlist => "Playlist",
            LoopStatus::Track => "Track",
            LoopStatus::None => "None",
        };
        return Value::Str(str.into());
    }
}