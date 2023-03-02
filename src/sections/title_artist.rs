use async_trait::async_trait;
use zbus::Result;

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent};
use super::Section;

pub struct TitleArtist<'a> {
    pub proxy: &'a SpotifyMediaPlayerProxy<'a>,
}

#[async_trait]
impl Section<'_> for TitleArtist<'_> {
    async fn update(&mut self, _click_event: &Option<ClickEvent>) -> Result<String> {
        let metadata = self.proxy.metadata().await?;

        Ok(format!("{} - {}", metadata.title, metadata.artist.get(0).unwrap()))
    }
}
