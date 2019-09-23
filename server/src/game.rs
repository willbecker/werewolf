use std::collections::{BTreeMap, HashSet, VecDeque};
use ws::CloseCode;

use crate::event::Event;
use crate::player::{Player, Role};

#[derive(PartialEq)]
enum State {
    Waiting, 
    RoleAssign,
    Shutdown,
}

enum Group {
    Role(Option<Role>),
    All,
    Host,
}

pub struct Game {
    lobby: BTreeMap<usize, Player>,
    state: State,
    missing_host: bool,
    waiting_for_client: bool,
    _ww_target: Option<usize>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            lobby: BTreeMap::new(),
            state: State::Waiting,
            missing_host: true,
            waiting_for_client: false,
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
                    Group::All,
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

    pub fn is_running(&mut self) -> bool {
        // Removes all players who have disconnected
        self.disconnect();

        if self.missing_host && self.lobby.len() > 0 {
            let new_host = self.lobby.values_mut().next().unwrap();
            new_host.assign_host();
            self.missing_host = false;
            println!("{} is now the host", new_host.name);
        }

        self.handle_event();

        match self.state {
            State::Waiting => self.waiting(),
            State::RoleAssign => self.role_assign(),
            State::Shutdown => return false,
        };
        true
    }

    // This will handle all of the events of the players and
    // should be used in a loop to check for player responses.
    fn handle_event(&mut self) {
        let mut event_que: VecDeque<(usize, Event)> = VecDeque::new();

        for (token, player) in self.lobby.iter_mut() {
            if let Ok(event) = player.event.try_recv() {
                event_que.push_back((*token, event)); 
            }
        }

        for (token, event) in event_que.iter() {
            match event {
                Event::Message(m) => println!("{}", m),
                Event::Ready => self.start_game(),
                Event::Disconnect => self.get_player(token).disconnect(),
                _ => (),
            };
        }
    }

    fn waiting(&mut self) {
        self.group_send(Group::All, Event::Waiting(self.lobby.len()));
        if self.lobby.len() >= 5 && !self.waiting_for_client {
            self.group_send(Group::Host, Event::Readycheck);
            self.waiting_for_client = true;
        }
    }

    fn start_game(&mut self) {
        let players = self.lobby.len();

        if players >= 5 {
            self.state = State::RoleAssign;
        } else {
            self.group_send(Group::Host,
                Event::Message(format!(
                    "Cannot start the game with only {} players",
                    players
                ))
            );
        }
    }

    fn role_assign(&mut self, ) {
        unimplemented!();
    }

    fn get_player(&mut self, token: &usize) -> &mut Player {
        self.lobby.get_mut(token).unwrap() 
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
                Group::All,
                Event::Message(format!(
                    "{} left the game",
                    self.lobby.get(&token).unwrap().name
                )),
            );

            if self.lobby.remove(&token).unwrap().is_host() {
                self.missing_host = true;
            }
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
    // Role(r) for all players with r role
    // All for all players
    // Host for the host player.
    fn group_send(&self, group: Group, event: Event) {
        match group {
            Group::Role(r) => { 
                for player in self.lobby.values().filter(|player| player.is_role(&r)) {
                    player.send(&event);
                }
            }
            Group::All => {
                for player in self.lobby.values() {
                    player.send(&event);
                }
            }
            Group::Host => {
                for player in self.lobby.values().filter(|player| player.is_host()) {
                    player.send(&event);
                }
            }
        }
    }

}
