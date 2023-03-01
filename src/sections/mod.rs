mod time_duration;

use std::future;

use async_trait::async_trait;
use zbus::Result;

use crate::dbus::SpotifyMediaPlayerProxy;

use self::time_duration::TimeDuration;

pub struct SectionList<'a> {
    sections: Vec<Box<dyn Section<'a> + 'a>>,
}

impl SectionList<'_> {
    pub async fn update(&self) -> Result<Vec<String>> {
        let parts = futures::future::join_all(
            self.sections.iter()
                .map(|s| s.update())
        ).await;

        return parts.into_iter().collect();
    }
}

pub fn init_sections<'a>(proxy: &'a SpotifyMediaPlayerProxy<'a>) -> SectionList<'a> {
    return SectionList {
        sections: vec![
            Box::new(
                TimeDuration { proxy }
            )
        ],
    };
}

#[async_trait]
trait Section<'a> {
    async fn update(&self) -> Result<String>;
}