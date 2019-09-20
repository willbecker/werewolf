use bincode;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Receiver;

use crate::game::Game;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Event {
    GetName,
    SetName(String),
    Message(String),
    Waiting(usize),
    Ready,
    Readycheck,
    Disconnect,
    List,
    Quit,
}

impl Into<Vec<u8>> for &Event {
    fn into(self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

pub fn handle_command_event(command: &Receiver<Event>, game: &mut Game) {
    if let Ok(event) = command.try_recv() {
        match event {
            Event::List => println!("{}", game.list()),
            Event::Quit => game.quit(),
            _ => (),
        };
    }
}
