mod dbus;
mod sections;
mod input;

use std::{time, thread};

use input::spawn_click_event_channel;
use sections::init_sections;
use zbus::{Connection, Result, CacheProperties};
use tokio;

use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    let proxy = dbus::SpotifyMediaPlayerProxy::builder(&connection)
        .cache_properties(CacheProperties::No)
        .build().await?;

    let mut args: Vec<_> = env::args().collect();

    // Remove the program arg
    args.remove(0);

    let mut sections = init_sections(&proxy, args);
    let click_event_channel = spawn_click_event_channel();

    loop {
        thread::sleep(time::Duration::from_millis(200));
        let click_event = click_event_channel.try_recv().ok();
        let strings = sections.update(click_event).await?;
        println!("{{\"full_text\": \"{}\"}}", strings.join(" | "));
    }
}
