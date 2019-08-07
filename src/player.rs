use ws::Sender;

pub struct Player {
    name: String,
    client: Sender,
}

impl Player {
    pub fn new(name: String, client: Sender) -> Player {
        Player { name, client }
    }
}
