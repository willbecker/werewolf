// A WebSocket echo server

use ws::*;

mod player;
use player::*;

struct Server {
    out: Sender,
    lobby: Vec<Player>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let connect_msg = format!(
            "New connection from {} with with token {}",
            shake.remote_addr().unwrap().unwrap(),
            usize::from(self.out.token())
        );
        println!("{}", connect_msg);
        self.out.send(Message::text("name_request"));

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("User {}: {}", usize::from(self.out.token()), msg);

        // echo message back to client showing who sent it
        let text = "User ".to_string()
            + &usize::from(self.out.token()).to_string()
            + ": "
            + &msg.into_text().unwrap();
        self.out.send(Message::text(text))
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
    println!("Server running at '127.0.0.1:3012'");
    listen("127.0.0.1:3012", |out| {
        let mut lobby = Vec::new();
        Server { out, lobby }
    })
    .unwrap();
}
