use zbus::{dbus_proxy, Result};

#[dbus_proxy(
    default_service = "org.bluez",
    interface = "org.bluez.Battery1",
)]
trait BluetoothBattery {
    #[dbus_proxy(property, name = "Percentage")]
    fn percent(&self) -> Result<u8>;
}
