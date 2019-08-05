extern crate rocket;

use rocket::http::Status;
use rocket::local::Client;

pub fn setup() {
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
}
