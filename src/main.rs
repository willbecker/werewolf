use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut roles = vec!["werewolf", "villager", "witch"];
    roles.shuffle(&mut rng);
    println!("{:?}", roles);
    for p in 0..roles.len(){
        println!("player {} is a {}", p+1, roles.pop().unwrap());
    }
}
