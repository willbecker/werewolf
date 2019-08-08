use ws::Sender;
use dialoguer::Input;

pub struct Player {
    name: String,
    client: Sender,
}

impl Player {
    pub fn new(name: String, client: Sender) -> Player {
        Player { name, client }
    }
}

fn promt_name() -> String {
    Input::<String>::new().with_prompt("Username").interact().unwrap()
}
