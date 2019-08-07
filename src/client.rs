use ws::*;
use bincode;
use dialoguer::Input;

mod utility;
use utility::*;

struct Client {
    server: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let name = promt_name();
        self.server.send(Message::binary(Packet::Username(name)));
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if msg.is_binary() {
            match bincode::deserialize(&msg.into_data()).unwrap() {
                Packet::Opt(event) => handle_event(event),
                p => println!("Bad Packet {:?}", p), 
            };
        } 
        else 
        {
            println!("{}", msg.into_text().unwrap());
        }
        Ok(())
    }
}

fn promt_name() -> String {
    Input::<String>::new().with_prompt("Username").interact().unwrap()
}

fn handle_event(event: Event) {
    unimplemented!();
}

fn main() {
    if let Err(error) = connect("ws://localhost:3012", |server| Client { server }) {
        println!("Failed to connect to WebSocket server ({:?})", error);
    }
}
