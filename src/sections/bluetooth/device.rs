use async_trait::async_trait;
use zbus::Result;

use crate::{input::ClickEvent, sections::{ReturnedResult, coloured_result, simple_result}, dbus::BluetoothDevice};
use super::super::Section;

pub struct BluetoothBar<'a> {
    pub proxy: BluetoothDevice<'a>,
}

#[async_trait]
impl Section<'_> for BluetoothBar<'_> {
    async fn update(&mut self, click_event: &Option<ClickEvent>) -> Result<ReturnedResult> {
        let name = self.proxy.controller.name().await?;
        let icon = self.proxy.controller.icon().await?;
        let connected = self.proxy.controller.connected().await?;

        if click_event.is_some() {
            if connected {
                self.proxy.controller.disconnect().await?;
            } else {
                self.proxy.controller.connect().await?;
            }
        }

        if !connected {
            return Ok(coloured_result(format!("{} {}", get_icon(icon), name), 0xFF0000));
        }

        let percent = self.proxy.battery.percent().await?;
        return Ok(simple_result(format!("{} {} ({}%)", get_icon(icon), name, percent)));
    }
}

fn get_icon(icon: String) -> String {
    match icon.as_str() {
        // https://fontawesome.com/icons/headphones?f=classic&s=solid
        "audio-headphones" | "audio-headset" => "\u{f025}",

        // https://fontawesome.com/icons/computer-mouse?f=classic&s=solid
        "input-mouse" => "\u{f8cc}",

        // https://fontawesome.com/icons/keyboard?f=classic&s=solid
        "input-keyboard" => "\u{f11c}",

        // https://fontawesome.com/icons/bluetooth?f=brands&s=solid
        _ => "\u{f293}"
    }.to_owned()
}

