// https://github.com/housleyjk/ws-rs/blob/master/examples/cli.rs
use bincode;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use ws::Sender as NetSender;
use ws::*;

pub fn server(sender: Sender<Connection>) {
    println!("Server running at '127.0.0.1:3012'");
    listen("127.0.0.1:3012", |out| Connection {
        out,
        thread: sender.clone(),
        event: Vec::new(),
    })
    .unwrap();
}

pub fn client(sender: Sender<Connection>) {
    if let Err(error) = connect("ws://localhost:3012", |out| Connection {
        out,
        thread: sender.clone(),
        event: Vec::new(),
    }) {
        println!("Failed to connect to WebSocket server ({:?})", error);
    }
}

#[derive(Clone)]
pub struct Connection {
    out: NetSender,
    thread: Sender<Connection>,
    event: Vec<Event>,
}

impl Handler for Connection {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let connect_msg = format!(
            "New connection from {} with with token {}",
            shake.remote_addr().unwrap().unwrap(),
            usize::from(self.out.token())
        );
        println!("{}", connect_msg);
        self.thread
            .send(self.clone())
            .expect("Could not send connection to main thread.");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if msg.is_binary() {
            let event: Event = bincode::deserialize(&msg.into_data()).unwrap();
            self.event.push(event);
        }
        Ok(())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!(
            "User {}: Disconnected {:?} ({})",
            usize::from(self.out.token()),
            code,
            reason
        );
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Event {
    SetName(String),
}

impl Into<Vec<u8>> for Event {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
