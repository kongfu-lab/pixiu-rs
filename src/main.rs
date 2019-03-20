#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use flaken::Flaken;


#[get("/")]
fn id() -> String  {
    let mut flake = Flaken::default().node(10).epoch(0).bitwidths(48, 12);
    return flake.next().to_string();
}


fn main() {
    rocket::ignite().mount("/", routes![id]).launch();
}