use zbus::{Result, Connection, CacheProperties};

use self::{controller::BluetoothControllerProxy, battery::BluetoothBatteryProxy};

pub mod battery;
pub mod controller;

pub struct BluetoothDevice<'a> {
    pub controller: BluetoothControllerProxy<'a>,
    pub battery: BluetoothBatteryProxy<'a>,
}

pub async fn bluetooth_proxy(mac: String) -> Result<BluetoothDevice<'static>> {
    let conn = &Connection::system().await?;
    let path = format!("/org/bluez/hci0/dev_{}", mac.replace(":", "_"));

    let controller = BluetoothControllerProxy::builder(conn)
        .path(path.clone())?
        .cache_properties(CacheProperties::No)
        .build().await?;

    let battery = BluetoothBatteryProxy::builder(conn)
        .path(path.clone())?
        .cache_properties(CacheProperties::No)
        .build().await?;

    Ok(BluetoothDevice { controller, battery })
}