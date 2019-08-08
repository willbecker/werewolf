use clap::{App, Arg};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

mod network;
use network::{client, server};

fn main() {
    let matches = App::new("Werewolf")
        .author("Will Becker <wibecker@pdx.edu>")
        .arg(Arg::with_name("server").short("s").help("Runs the server"))
        .get_matches();

    if matches.is_present("server") {
        let (sender, receiver) = channel();

        thread::spawn(move || server(sender));

        loop {
            thread::sleep(Duration::from_millis(10));
        }
    } else {
        let (sender, receiver) = channel();

        thread::spawn(move || client(sender));

        loop {
            thread::sleep(Duration::from_millis(10));
        }
    }
}
