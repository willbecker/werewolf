// https://github.com/housleyjk/ws-rs/blob/master/examples/cli.rs
use bincode;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use ws::Sender as NetSender;
use ws::*;

use crate::event::Event;

pub fn server(sender: Sender<(NetSender, Receiver<Event>)>) {
    println!("Server running at '127.0.0.1:3012'");
    listen("127.0.0.1:3012", |out| Connection {
        out,
        thread: sender.clone(),
        event_que: None,
    })
    .unwrap();
}

pub fn client(sender: Sender<(NetSender, Receiver<Event>)>, url: String) {
    if let Err(error) = connect(url, |out| Connection {
        out,
        thread: sender.clone(),
        event_que: None,
    }) {
        println!("Failed to connect to server ({:?})", error);
    }
}

#[derive(Clone)]
pub struct Connection {
    pub out: NetSender,
    thread: Sender<(NetSender, Receiver<Event>)>,
    pub event_que: Option<Sender<Event>>,
}

impl Handler for Connection {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let (event_send, event_recv) = channel();
        self.event_que = Some(event_send);
        self.thread
            .send((self.out.clone(), event_recv))
            .expect("Could not send connection to main thread.");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if msg.is_binary() {
            let event: Event = bincode::deserialize(&msg.into_data()).unwrap();
            self.event_que
                .as_ref()
                .unwrap()
                .send(event)
                .expect("Could not send event to main thread");
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
