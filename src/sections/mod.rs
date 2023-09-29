mod spotify;
mod bluetooth;

use async_trait::async_trait;
use zbus::{Result, Error, Connection};

use crate::{dbus::{spotify_proxy, bluetooth_proxy}, input::ClickEvent};

use spotify::{time_progress_bar::TimeProgressBar, title_artist::TitleArtist, bpm::BPM};

use self::bluetooth::device::BluetoothBar;

pub struct ReturnedResult {
    pub text: String,
    pub colour: i32,
}
pub fn simple_result(text: String) -> ReturnedResult { ReturnedResult { text, colour: -1 } }
pub fn coloured_result(text: String, colour: i32) -> ReturnedResult { ReturnedResult { text, colour } }

#[async_trait]
pub trait Section<'a> {
    async fn update(&mut self, click_event: &Option<ClickEvent>) -> Result<ReturnedResult>;
}

pub async fn init_sections<'a>(bar: String, mut args: Vec<String>) -> Result<Box<dyn Section<'a> + 'a>> {
    return match bar.as_str() {
        "spotify-bpm" => Ok(Box::new( BPM {
            proxy: spotify_proxy().await?,
            last_searched: None,
        } )),
        "spotify-title" => Ok(Box::new( TitleArtist {
            proxy: spotify_proxy().await?,
        } )),
        "spotify-progress" => Ok(Box::new( TimeProgressBar {
            proxy: spotify_proxy().await?,
            width: 20,
        } )),
        "bluetooth" => Ok(Box::new( BluetoothBar {
            proxy: bluetooth_proxy(args.remove(0)).await?,
        } )),
        _ => Err(Error::Failure("Unknown bar {b}".to_owned())),
    };
}
