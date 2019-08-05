// A WebSocket echo server

use ws::*;

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("New connection from {} with with token {}",
                 shake.remote_addr().unwrap().unwrap(),
                 usize::from(self.out.token()));
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
        println!("User {}: Disconnected {:?} ({})",
            usize::from(self.out.token()),
            code,
            reason);
    }
}

fn main() {
    println!("Server running at '127.0.0.1:3012'");
    listen("127.0.0.1:3012", |out| { Server {out} }).unwrap()
}
