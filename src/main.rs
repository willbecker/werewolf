#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::State;

mod roles;
use roles::*;


#[get("/")]
fn index(roles: State<RoleGen>) -> String {
   roles.get(0)
}

fn main() {
    rocket::ignite()
        .manage(RoleGen::new())
        .mount("/", routes![index]).launch();

}
