// A WebSocket echo server

use ws::*;
use bincode;
use std::cell::RefCell;
use std::rc::Rc;

mod player;
mod utility;
use player::*;
use utility::*;

struct Server {
    out: Sender,
    lobby: Rc<RefCell<Vec<Player>>>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let connect_msg = format!(
            "New connection from {} with with token {}",
            shake.remote_addr().unwrap().unwrap(),
            usize::from(self.out.token())
        );
        println!("{}", connect_msg);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if msg.is_binary() {
            match bincode::deserialize(&msg.into_data()).unwrap() {
                Packet::Username(name) => 
                    self.lobby.borrow_mut().push(Player::new(name, self.out.clone())),
                p => println!("Bad Packet {:?}", p),
            }
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

fn main() {
    let mut main_lobby = Rc::new(RefCell::new(Vec::new()));
    println!("Server running at '127.0.0.1:3012'");
    listen("127.0.0.1:3012", |out| {
        Server { 
            out, 
            lobby: main_lobby.clone(),
        }
    })
    .unwrap();
}
