mod dbus;
mod sections;
mod input;

use std::{time, thread, path::Component, arch::x86_64::CpuidResult};

use input::spawn_click_event_channel;
use sections::init_sections;
use zbus::{Connection, Error, Result, CacheProperties};
use tokio;

use std::env;

const CHECK_INTERVAL: u64 = 50;
const NUM_FAILED_FORCE_RENDER: i32 = 4;

const ERROR_WAIT_TIMEOUT: u64 = 500;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    loop {
        match main_loop().await {
            Ok(_) => Err(Error::Failure("Should not finish the main loop".to_owned())),
            Err(err) => {
                println!("Encountered err {}, retrying", err);
                thread::sleep(time::Duration::from_millis(ERROR_WAIT_TIMEOUT));
                Ok(())
            }
        }?

    }
}


async fn main_loop() -> Result<()> {
    let mut args: Vec<_> = env::args().collect();

    // Remove the program arg
    args.remove(0);

    let bar = args.remove(0);

    let mut section = init_sections(bar, args).await?;
    let click_event_channel = spawn_click_event_channel();

    loop {
        for i in 0..NUM_FAILED_FORCE_RENDER {
            thread::sleep(time::Duration::from_millis(CHECK_INTERVAL));
            let click_event = click_event_channel.try_recv().ok();
            if i == 0 || click_event.is_some() {
                let result = section.update(&click_event).await?;
                println!("{{\"full_text\": \"{}\", \"color\": \"#{:x}\"}}", result.text, result.colour);
            }
        }
    }
}
