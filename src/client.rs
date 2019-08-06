use ws::{connect, Handler, Message, Result, Sender};

struct Client {
    server: Sender,
}

impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg.into_text().unwrap());
        Ok(())
    }
}

fn main() {
    if let Err(error) = connect("ws://localhost:3012", |server| Client { server }) {
        println!("Failed to connect to WebSocket server ({:?})", error);
    }
}
