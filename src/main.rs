use clap::{App, Arg};
use dialoguer::theme::CustomPromptCharacterTheme;
use dialoguer::Input;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use ws::Message;

mod network;
use network::{client, server};

mod client;
use client::Client;

mod event;
use event::*;

mod player;
use player::Player;

mod game;
use game::Game;

type Prompt = CustomPromptCharacterTheme;

// The main function parses the command line argumens using Clap
// to determine if the server or client should be run. By default,
// the client will be run. Use "--server" or "-s" to run the server.
// "-h" will give the standard help message.
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

        // The listener needs to run on its own thread to pick up
        // new clients while the game is running. The listener sends
        // back info about those new clients using a channel.
        // client_recv will contain a tuple with a Sender for the client,
        // and an Event Receiver that we can use to check for Events
        // coming from the client.
        let (client_send, client_recv) = channel();
        thread::spawn(move || server(client_send));

        // We want a command prompt so that we can interact with the
        // server while it is running. This will need to be on a seperate
        // thread so that game can run while we enter commands. The prompt
        // will send Events back over a channel.
        let (command_send, command_recv) = channel();
        thread::spawn(move || command_promt(command_send));

        let mut new_players = HashSet::new();
        let mut game = Game::new();

        while game.is_running() {
            // First, check to see if there are any new clients sitting
            // in the receiver. For each client found, send a username
            // request, create a Player with the client info, and add
            // that player to the list of new players.
            let connections = client_recv.try_iter();
            for (client, event_recv) in connections {
                println!("Creating Player {:?}", client.token());
                client.send(Message::binary(&Event::GetName)).unwrap();
                new_players.insert(Player::new(client, event_recv));
            }

            new_players = game.add_players(new_players);

            game.run();

            handle_command_event(&command_recv, &mut game);

            thread::sleep(Duration::from_millis(10));
        }
    } else {
        // CLIENT

        // The client's outer loop will keep prompting for a server
        // address until it makes a connection.
        loop {
            // The prompt defaults to localhost to make my life
            // easier when testing. I plan to make this more general
            // in the future.
            let url = Input::<String>::new()
                .with_prompt("Server address")
                .default("ws:/127.0.0.1:3012".to_string())
                .interact()
                .unwrap();

            // Use a channel to retreive server info from the connection
            // handler thread.
            let (server_send, server_recv) = channel();
            let handle = thread::spawn(move || client(server_send, url));

            // If we get the server info back in a reasonable amount of time,
            // we create a Client passing the server info to it. The inner loop
            // runs as long as the event handler is successful. Sleep is uesed
            // to control the polling rate of the event handler.
            if let Ok(server) = server_recv.recv_timeout(Duration::from_secs(10)) {
                let mut client = Client::new(server.0, server.1);

                while client.handle_event() {
                    thread::sleep(Duration::from_millis(10));
                }
            }

            // If the connection fails or is dropped at any point, then we
            // wait for the connection handler thread to finish up and move
            // to the next iteration of the outer loop.
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
