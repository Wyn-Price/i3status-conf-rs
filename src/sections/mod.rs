mod time_progress_bar;
mod title_artist;
mod bpm;

use async_trait::async_trait;
use zbus::{Result, Error};

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent};

use self::{time_progress_bar::TimeProgressBar, title_artist::TitleArtist, bpm::BPM};

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

pub fn init_sections<'a>(proxy: &'a SpotifyMediaPlayerProxy<'a>, blocks: Vec<String>) -> Result<SectionList<'a>> {
    let sections_res: Result<Vec<Box<dyn Section<'_>>>> = blocks.iter().map(|b| -> Result<Box<dyn Section<'_>>> {
        return match b.as_str() {
            "bpm" => Ok(Box::new( BPM { proxy, last_searched: None } )),
            "title" => Ok(Box::new( TitleArtist { proxy } )),
            "progress" => Ok(Box::new( TimeProgressBar { width: 20, proxy } )),
            _ => Err(Error::Failure(format!("Unknown bar {b}"))),
        };
    }).collect();

    let sections = sections_res?;

    if sections.len() == 0 {
        return Err(Error::Failure("No sections specified".to_owned()));
    }

    return Ok(SectionList { sections });
}
