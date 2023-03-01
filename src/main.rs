mod dbus;
mod sections;

use std::{time, thread};

use sections::init_sections;
use zbus::{Connection, Result, CacheProperties};
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    let proxy = dbus::SpotifyMediaPlayerProxy::builder(&connection)
        .cache_properties(CacheProperties::No)
        .build().await?;
    
    let sections = init_sections(&proxy);
    
    loop {
        thread::sleep(time::Duration::from_millis(200));
        let strings = sections.update().await?;
        println!("{}", strings.join(" "));
    }
}