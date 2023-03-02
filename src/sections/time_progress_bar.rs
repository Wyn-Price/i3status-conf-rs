use async_trait::async_trait;
use zbus::Result;

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
    async fn update(&mut self, _click_event: &Option<ClickEvent>) -> Result<String> {
        let position = self.proxy.position().await? as f64;
        let total = self.proxy.metadata().await?.track_length as f64;

        let progress = self.width as f64 * (position / total);
        let mut str = PROGRESS_CHARS[NUM_CHARS - 1].to_string().repeat(progress as usize);

        let fract = progress.fract();
        str += &PROGRESS_CHARS[(fract * NUM_CHARS as f64).floor() as usize].to_string();

        if (progress as usize) < (self.width as usize - 1) {
            str += &EMPTY_CHARACTER.to_string().repeat(self.width as usize - progress as usize - 1);
        }

        Ok(format!("{}[{}]{}", convert_time(position), str, convert_time(total)))
    }
}

fn convert_time(time: f64) -> String {
    let minutes = time as i64 / (1000000 * 60);
    let seconds = (time as i64 / 1000000) - (minutes * 60);

    return format!("{:0>1}:{:0>2}", minutes, seconds);
}
