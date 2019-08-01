use rand::prelude::*;

pub struct RoleGen {
    roles: Vec<String>,
}

impl RoleGen {
    pub fn new() -> RoleGen {
        let mut rng = rand::thread_rng();
        let mut roles = vec![
            "werewolf".to_string(),
            "villager".to_string(),
            "witch".to_string()
        ];

        roles.shuffle(&mut rng);

        RoleGen {
            roles: roles,
        }
    }

    pub fn get(&self, num: usize) -> String {
        self.roles[num].clone()
    }
}

