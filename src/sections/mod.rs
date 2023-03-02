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

pub fn init_sections<'a>(proxy: &'a SpotifyMediaPlayerProxy<'a>, mut blocks: Vec<String>) -> SectionList<'a> {
    if blocks.is_empty() {
        blocks.append(&mut vec![
            "title".to_owned(),
            "progress".to_owned()
        ]);
    }
    return SectionList {
        sections: blocks.iter().map(|b| {
            let b: Box<dyn Section> = match b.as_str() {
                "title" => Box::new( TitleArtist { proxy } ),
                "progress" => Box::new( TimeProgressBar { width: 20, proxy } ),
                _ => {
                    println!("Unknown bar {b}");
                    panic!("Unknown bar {b}");
                }
            };
            b
        }).collect()
    };
}
