use std::sync::mpsc::Receiver;
use std::hash::{Hash, Hasher};
use ws::Sender as NetSender;
use ws::Message;

use crate::event::Event;

#[derive(PartialEq)]
pub enum Role {
    Werewolf,
    Villager,
    Witch,
}

pub struct Player {
    pub client: NetSender,
    pub event: Receiver<Event>,
    pub name: String,
    role: Option<Role>,
}

impl Player {
    pub fn new(client: NetSender, event_recv: Receiver<Event>) -> Player {
        Player {
            client,
            event: event_recv,
            name: "".to_string(),
            role: None,
        }
    }
    
    pub fn send(&self, event: &Event) {
        self.client.send(Message::binary(event)).unwrap();
    }

    pub fn is_role(&self, role: &Option<Role>) -> bool {
        self.role == *role
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let token = usize::from(self.client.token());
        token.hash(state);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.client == other.client
    }
}

impl Eq for Player {}
