use dialoguer::Input;
use std::sync::mpsc::Receiver;
use ws::Message;
use ws::Sender as NetSender;

use crate::event::Event;

pub struct Client {
    server: NetSender,
    event: Receiver<Event>,
}

impl Client {
    pub fn new(server: NetSender, event: Receiver<Event>) -> Client {
        Client { server, event }
    }

    pub fn send(&self, event: Event) {
        self.server.send(Message::binary(&event)).unwrap();
    }

    // return true when the event gets received.
    // return false if we disconnect.
    pub fn handle_event(&mut self) -> bool {
        if let Ok(event) = self.event.recv() {
            match event {
                Event::GetName => self.prompt_name(),
                Event::Message(m) => println!("{}", m),
                _ => (),
            };
            return true;
        }
        false
    }

    fn prompt_name(&self) {
        let name = Input::<String>::new()
            .with_prompt("Username")
            .interact()
            .unwrap();

        self.send(Event::SetName(name));
    }
}
