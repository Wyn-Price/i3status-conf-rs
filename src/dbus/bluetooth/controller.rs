use zbus::{dbus_proxy, Result};

#[dbus_proxy(
    default_service = "org.bluez",
    interface = "org.bluez.Device1",
)]
trait BluetoothController {
    #[dbus_proxy(name = "Connect")]
    async fn connect(&self) -> Result<()>;

    #[dbus_proxy(name = "Disconnect", no_reply)]
    async fn disconnect(&self) -> Result<()>;

    #[dbus_proxy(property, name = "Name")]
    fn name(&self) -> Result<String>;

    #[dbus_proxy(property, name = "Icon")]
    fn icon(&self) -> Result<String>;

    #[dbus_proxy(property, name = "Connected")]
    fn connected(&self) -> Result<bool>;
}
