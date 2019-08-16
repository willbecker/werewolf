use std::collections::{BTreeMap, HashSet};
use ws::CloseCode;

use crate::event::Event;
use crate::player::{Player, Role};

// each variant has a bool indicate if it has already been
// run. True means that we are on the first cycle of the
// state, while false means that we are on some later cycle.
#[derive(PartialEq)]
enum State {
    Waiting(bool),
    Shutdown,
}

pub struct Game {
    lobby: BTreeMap<usize, Player>,
    state: State,
    _ww_target: Option<usize>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            lobby: BTreeMap::new(),
            state: State::Waiting(true),
            _ww_target: None,
        }
    }

    // Check to see if we received a username for any of the players.
    // If we did, set their username, then add them to the game. For
    // all players that we haven't received a username, give them
    // back to main.
    //
    // Using the sender token as a key seems like good idea because
    // every client gets unique token (starting from 0) when they
    // connect. If a player disconnects, their token is free to be
    // used by new connecting players.
    pub fn add_players(&mut self, mut players: HashSet<Player>) -> HashSet<Player> {
        let mut unnamed = HashSet::new();
        for mut player in players.drain() {
            if let Ok(Event::SetName(name)) = player.event.try_recv() {
                player.name = name;
                self.group_send(
                    None,
                    Event::Message(format!("{} joined the game", &player.name)),
                );
                self.lobby
                    .insert(usize::from(player.client.token()), player);
            } else {
                unnamed.insert(player);
            }
        }
        unnamed
    }

    pub fn run(&mut self) -> bool {
        // Removes all players who have disconnected
        self.disconnect();

        self.handle_event();

        self.state = match self.state {
            State::Waiting(first) => self.waiting(first),
            State::Shutdown => State::Shutdown,
        };
        false
    }

    // This will handle all of the events of the players and
    // should be used in a loop to check for player responses.
    fn handle_event(&mut self) {
        for (_token, player) in self.lobby.iter_mut() {
            if let Ok(event) = player.event.try_recv() {
                match event {
                    Event::Message(m) => println!("{}", m),
                    Event::Disconnect => player.disconnect(),
                    _ => (),
                };
            }
        }
    }

    // Check all players to see if they have disconnected, then
    // remove all players who have.
    fn disconnect(&mut self) {
        let mut to_remove = Vec::new();
        for (token, player) in self.lobby.iter() {
            if !player.connected {
                to_remove.push(*token);
            }
        }
        for token in to_remove {
            self.group_send(
                None,
                Event::Message(format!(
                    "{} left the game",
                    self.lobby.get(&token).unwrap().name
                )),
            );
            self.lobby.remove(&token);
        }
    }

    // Print a total player count, and a formatted list of
    // each player and their token.
    pub fn list(&self) -> String {
        let mut list = format!("Player count: {}\n", self.lobby.len());
        for (token, player) in self.lobby.iter() {
            list += format!("{}: {}\n", token, player.name).as_str();
        }
        list
    }

    pub fn is_running(&mut self) -> bool {
        self.state != State::Shutdown
    }

    // Disconnect all of the players and set state to Shutdown
    pub fn quit(&mut self) {
        for player in self.lobby.values() {
            player
                .client
                .close_with_reason(CloseCode::Normal, "Server shutdown")
                .unwrap();
        }
        self.state = State::Shutdown;
    }

    // Send an event to a specific group of players
    // None means all player will receive the event.
    fn group_send(&self, group: Option<Role>, event: Event) {
        if group.is_none() {
            for player in self.lobby.values() {
                player.send(&event);
            }
        } else {
            for player in self.lobby.values().filter(|player| player.is_role(&group)) {
                player.send(&event);
            }
        }
    }

    fn waiting(&mut self, _first: bool) -> State {
        self.group_send(None, Event::Waiting(self.lobby.len()));
        State::Waiting(false)
    }
}
