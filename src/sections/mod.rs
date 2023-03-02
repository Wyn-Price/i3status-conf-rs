mod time_progress_bar;
mod title_artist;

use async_trait::async_trait;
use zbus::Result;

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent};

use self::{time_progress_bar::TimeProgressBar, title_artist::TitleArtist};

pub struct SectionList<'a> {
    sections: Vec<Box<dyn Section<'a> + 'a>>,
}

#[async_trait]
trait Section<'a> {
    async fn update(&mut self, click_event: &Option<ClickEvent>) -> Result<String>;
}

impl SectionList<'_> {
    pub async fn update(&mut self, click_event: Option<ClickEvent>) -> Result<Vec<String>> {
        let parts = futures::future::join_all(
            self.sections.iter_mut()
                .map(|s| s.update(&click_event))
        ).await;

        return parts.into_iter().collect();
    }
}

pub fn init_sections<'a>(proxy: &'a SpotifyMediaPlayerProxy<'a>) -> SectionList<'a> {
    return SectionList {
        sections: vec![
            Box::new( TitleArtist { proxy } ),
            Box::new( TimeProgressBar { width: 20, proxy } ),
        ],
    };
}
