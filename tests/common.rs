extern crate rocket;
use moneytransfer;

use rocket::http::Status;
use rocket::local::Client;

// pub fn setup() {
//     let rocket = rocket::ignite();
//     let client = Client::new(rocket).expect("valid rocket instance");
//     let mut response = client.get("/").dispatch();
// }

#[test]
pub fn sanity_check() {
   let client = Client::new(moneytransfer::rocket())
    .expect("valid rocket instance");

    let mut response = client.get("/sanity").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("i am sane".into()));
}

