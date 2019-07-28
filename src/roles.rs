use rand::prelude::*;

pub const AVAILABLE_ROLES: [&str; 3] = [
    "werewolf",
    "villager",
    "witch",
];

pub enum Role {
    Werewolf, 
    Villager, 
    Witch,
}

pub fn print_roles() {
    let mut rng = rand::thread_rng();
    let mut roles = vec!["h", "j"];

    roles.shuffle(&mut rng);
    println!("{:?}", roles);
    for p in 0..roles.len(){
        println!("player {} is a {}", p+1, roles.pop().unwrap());
    }
}
