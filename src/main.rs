mod dbus;

use zbus::{Connection, Result};


// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = dbus::SpotifyMediaPlayerProxy::new(&connection).await?;
    // proxy.play_pause().await?;
    Ok(())
}