use std::sync::mpsc::Receiver;
use ws::Message;
use ws::Sender as NetSender;

use crate::event::Event;
use crate::terminal;

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
                Event::GetName => {
                    let name = terminal::prompt("Username");
                    self.send(Event::SetName(name));
                }
                Event::Message(m) => terminal::print_pos(1, 1, m),
                Event::Readycheck => self.ready_check(),
                Event::Waiting(c) => self.waiting(c),
                _ => (),
            };
            return true;
        }
        false
    }

    fn waiting(&mut self, player_count: usize) {
        terminal::print_pos(1, 2, "5 players are required to start the game".to_string());
        terminal::print_pos(1, 3, format!("Players [{}/5]", player_count));
    }

    fn ready_check(&self) {
        terminal::confirm("Press Enter to start the game");
        self.send(Event::Ready);
    }
}
