use std::fmt::format;

use async_trait::async_trait;
use zbus::Result;
use unicode_segmentation::UnicodeSegmentation;

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent};
use super::Section;

const NUM_CHARS: usize = 8;
const PROGRESS_CHARS: [char; NUM_CHARS] = [
    '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'
];
const EMPTY_CHARACTER: char = ' ';

pub struct TimeProgressBar<'a> {
    pub proxy: &'a SpotifyMediaPlayerProxy<'a>,
    pub width: u8,
}

#[async_trait]
impl Section<'_> for TimeProgressBar<'_> {
    async fn update(&mut self, click_event: &Option<ClickEvent>) -> Result<String> {
        let position = self.proxy.position().await? as f64;
        let total = self.proxy.metadata().await?.track_length as f64;

        let progress = self.width as f64 * (position / total);
        let mut str = PROGRESS_CHARS[NUM_CHARS - 1].to_string().repeat(progress as usize);

        let fract = progress.fract();
        str += &PROGRESS_CHARS[(fract * NUM_CHARS as f64).floor() as usize].to_string();

        if (progress as usize) < (self.width as usize - 1) {
            str += &EMPTY_CHARACTER.to_string().repeat(self.width as usize - progress as usize - 1);
        }

        let prefix = format!("{}[", convert_time(position));
        let suffix = format!("]{}", convert_time(total));

        let prefix_size = prefix.graphemes(true).count();
        let suffix_size = suffix.graphemes(true).count();

        if let Some(click) = click_event {
            let pixels_per_char = click.width / click.full_text.graphemes(true).count();

            let pixels_prefix = prefix_size * pixels_per_char;
            let pixels_suffix = suffix_size * pixels_per_char;

            if click.relative_x < pixels_prefix {
                self.proxy.previous().await?;
            } else if click.relative_x > click.width - pixels_suffix {
                self.proxy.next().await?;
            } else {
                let progress = (click.relative_x - pixels_prefix) as f64 / (click.width - pixels_suffix - pixels_prefix) as f64;

                let target = (progress * total) as i64;
                let current = position as i64;

                // The seek function seems to add one second, maybe try and use the `SetOffset` function?
                self.proxy.seek_plus_one_second(target - current - 1000000).await?;
            }
        }

        Ok(format!("{}{}{}", prefix, str, suffix))
    }
}

fn convert_time(time: f64) -> String {
    let minutes = time as i64 / (1000000 * 60);
    let seconds = (time as i64 / 1000000) - (minutes * 60);

    return format!("{:0>1}:{:0>2}", minutes, seconds);
}
