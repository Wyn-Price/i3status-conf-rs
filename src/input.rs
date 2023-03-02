use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use serde::Deserialize;

pub fn spawn_click_event_channel() -> Receiver<ClickEvent> {
    let (tx, rx) = mpsc::channel::<ClickEvent>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match serde_json::from_str(&buffer) {
            Ok(click_event) => tx.send(click_event).unwrap(),
            Err(err) => panic!("Invalid stdin {}, {}", &buffer, err),
        }
    });
    rx
}

#[derive(Debug, Deserialize)]
pub struct ClickEvent {
    button: u8,
    modifiers: Vec<String>,
    x: usize,
    y: usize,
    relative_x: usize,
    relative_y: usize,
    output_x: usize,
    output_y: usize,
    width: usize,
    height: usize,
}
