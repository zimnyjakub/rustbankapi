#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod domain;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![routes::transfers::schedule])
}

fn main() {
    rocket().launch();
}
