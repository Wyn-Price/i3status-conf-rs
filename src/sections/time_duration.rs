use std::fmt::format;

use async_trait::async_trait;
use zbus::Result;

use crate::dbus::SpotifyMediaPlayerProxy;

use super::Section;

pub struct TimeDuration<'a> {
    pub proxy: &'a SpotifyMediaPlayerProxy<'a>,
}

#[async_trait]
impl Section<'_> for TimeDuration<'_> {
    async fn update(&self) -> Result<String> {
        let position = self.proxy.position().await?;
        let total = self.proxy.metadata().await?.track_length as i64;

        return Ok(format!("{}/{}", convert_time(position), convert_time(total)));
    }
}

fn convert_time(time: i64) -> String {
    let minutes = time / (1000000 * 60);
    let seconds = (time / 1000000) - (minutes * 60);

    return format!("{:0>1}:{:0>2}", minutes, seconds);
}