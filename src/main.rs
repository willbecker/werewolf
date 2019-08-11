use clap::{App, Arg};
use dialoguer::theme::CustomPromptCharacterTheme;
use dialoguer::Input;
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use ws::{CloseCode, Message};

mod network;
use network::{client, server};

mod event;
use event::*;

mod player;
use player::Player;

type Prompt = CustomPromptCharacterTheme;

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
        // SERVER
        let (client_send, client_recv) = channel();
        thread::spawn(move || server(client_send));

        let (command_send, command_recv) = channel();
        thread::spawn(move || command_promt(command_send));

        let mut lobby = BTreeMap::new();

        loop {
            let connections = client_recv.try_iter();
            for (client, event_recv) in connections {
                println!("Creating Player {:?}", client.token());
                client.send(Message::binary(Event::GetName)).unwrap();
                lobby.insert(usize::from(client.token()), Player::new(client, event_recv));
            }

            handle_player_event(&mut lobby);

            if handle_command_event(&command_recv, &mut lobby) {
                for player in lobby.values() {
                    player
                        .client
                        .close_with_reason(CloseCode::Normal, "Server shutdown")
                        .unwrap();
                }
                thread::sleep(Duration::from_millis(100));
                return;
            }

            thread::sleep(Duration::from_millis(10));
        }
    } else {
        // CLIENT
        loop {
            let (sender, receiver) = channel();
            let url = Input::<String>::new()
                .with_prompt("Server address")
                .default("ws:/127.0.0.1:3012".to_string())
                .interact()
                .unwrap();

            let handle = thread::spawn(move || client(sender, url));

            if let Ok(server) = receiver.recv_timeout(Duration::from_secs(10)) {
                while handle_server_event(&server).is_ok() {
                    thread::sleep(Duration::from_millis(10));
                }
            }
            handle.join().unwrap();
        }
    }
}

fn command_promt(main_thread: Sender<Event>) {
    while {
        let mut event = None;
        while event.is_none() {
            thread::sleep(Duration::from_millis(100));
            let command = Input::<String>::with_theme(&Prompt::new('>'))
                .interact()
                .unwrap();

            match command.as_ref() {
                "list" => event = Some(Event::List),
                "quit" => event = Some(Event::Quit),
                _ => println!("Invalid command ({})", command),
            };
        }
        main_thread.send(event.unwrap()).is_ok()
    } {} // do while: https://gist.github.com/huonw/8435502
}
