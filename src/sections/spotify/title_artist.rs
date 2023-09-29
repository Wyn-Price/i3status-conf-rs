use async_trait::async_trait;
use zbus::Result;

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent, sections::{simple_result, ReturnedResult}};
use super::super::Section;

pub struct TitleArtist<'a> {
    pub proxy: SpotifyMediaPlayerProxy<'a>,
}

#[async_trait]
impl Section<'_> for TitleArtist<'_> {
    async fn update(&mut self, _click_event: &Option<ClickEvent>) -> Result<ReturnedResult> {
        let metadata = self.proxy.metadata().await?;

        Ok(simple_result(format!("{} - {}", metadata.title, metadata.artist.get(0).unwrap())))
    }
}
