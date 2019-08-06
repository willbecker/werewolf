use ws::Sender;

pub struct Player {
    name: String,
    client: Sender,
}

impl Player {
    pub fn new(client: Sender) -> Player {
        let name = "".to_string();

        Player { name, client }
    }
}
