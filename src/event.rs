use bincode;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use ws::util::Token;
use ws::Message;
use ws::Sender as NetSender;

use crate::player::Player;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Event {
    GetName,
    SetName(String),
    Message(String),
}

impl Into<Vec<u8>> for Event {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

pub fn handle_player_event(players: &mut HashMap<Token, Player>) {
    for player in players.values_mut() {
        if let Ok(event) = player.event_recv.try_recv() {
            match event {
                Event::SetName(name) => player.name = name,
                Event::Message(m) => println!("{}", m),
                _ => (),
            };
        }
    }
}

pub fn handle_server_event(server: &(NetSender, Receiver<Event>)) {
    if let Ok(event) = server.1.try_recv() {
        match event {
            Event::GetName => promt_name(&server.0),
            Event::Message(m) => println!("{}", m),
            _ => (),
        };
    }
}

fn promt_name(server: &NetSender) {
    let name = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .unwrap();
    server.send(Message::binary(Event::SetName(name))).unwrap();
}
