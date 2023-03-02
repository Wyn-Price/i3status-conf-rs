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
    pub button: u8,
    pub full_text: String,
    pub modifiers: Vec<String>,
    pub x: usize,
    pub y: usize,
    pub relative_x: usize,
    pub relative_y: usize,
    pub output_x: usize,
    pub output_y: usize,
    pub width: usize,
    pub height: usize,
}
