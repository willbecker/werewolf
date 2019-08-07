use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    GetName,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Username(String),
    Opt(Event),
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}   
