use crate::event::Event;
use std::sync::mpsc::Receiver;
use ws::Sender as NetSender;

pub struct Player {
    pub client: NetSender,
    pub event_recv: Receiver<Event>,
    pub name: String,
}

impl Player {
    pub fn new(client: NetSender, event_recv: Receiver<Event>) -> Player {
        Player {
            client,
            event_recv,
            name: "".to_string(),
        }
    }
}
