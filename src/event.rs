use bincode;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, TryRecvError};
use ws::Message;
use ws::Sender as NetSender;

use crate::player::Player;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Event {
    GetName,
    SetName(String),
    Message(String),
    List,
    Quit,
}

impl Into<Vec<u8>> for Event {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

pub fn handle_player_event(players: &mut BTreeMap<usize, Player>) {
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

pub fn handle_command_event(
    command: &Receiver<Event>,
    players: &mut BTreeMap<usize, Player>,
) -> bool {
    let mut quit = false;
    if let Ok(event) = command.try_recv() {
        match event {
            Event::List => list(players),
            Event::Quit => quit = true,
            _ => (),
        };
    }
    quit
}

pub fn handle_server_event(server: &(NetSender, Receiver<Event>)) -> Result<(), TryRecvError> {
    let event = server.1.try_recv();
    if event.is_ok() {
        match event.unwrap() {
            Event::GetName => prompt_name(&server.0),
            Event::Message(m) => println!("{}", m),
            _ => (),
        };
    } else if event == Err(TryRecvError::Disconnected) {
        return Err(TryRecvError::Disconnected);
    }
    Ok(())
}

fn list(players: &mut BTreeMap<usize, Player>) {
    for (token, player) in players.iter() {
        println!("{}: {}", token, player.name);
    }
}
fn prompt_name(server: &NetSender) {
    let name = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .unwrap();
    server.send(Message::binary(Event::SetName(name))).unwrap();
}
