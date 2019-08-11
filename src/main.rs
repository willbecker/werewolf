use clap::{App, Arg};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use ws::Message;

mod network;
use network::{client, server};

mod event;
use event::*;

mod player;
use player::Player;

fn main() {
    let matches = App::new("Werewolf")
        .author("Will Becker <wibecker@pdx.edu>")
        .arg(
            Arg::with_name("server")
                .long("server")
                .short("s")
                .help("Runs the server"),
        )
        .get_matches();

    if matches.is_present("server") {
        // Code for running the server
        let (sender, receiver) = channel();
        let handle = thread::spawn(move || server(sender));

        let mut lobby = HashMap::new();

        loop {
            let connections = receiver.try_iter();
            for (client, event_recv) in connections {
                println!("Creating Player {:?}", client.token());
                client.send(Message::binary(Event::GetName)).unwrap();
                lobby.insert(client.token(), Player::new(client, event_recv));
            }

            handle_player_event(&mut lobby);

            thread::sleep(Duration::from_millis(10));
        }
    } else {
        // Code for running the client
        let (sender, receiver) = channel();
        let handle = thread::spawn(move || client(sender));

        let mut server = receiver
            .recv_timeout(Duration::from_secs(2))
            .expect("Error");

        loop {
            handle_server_event(&server);

            thread::sleep(Duration::from_millis(10));
        }
    }
}
