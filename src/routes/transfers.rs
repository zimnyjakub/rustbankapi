use rocket_contrib::uuid::Uuid;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Transfer {
    from: String,
    to: String,
    amount: i64,
    currency: String
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "from {} to {} amount {} ccy {} ", self.from, self.to, self.amount, self.currency)
        }
}

#[post("/transfer", data = "<transfer>")]
pub fn schedule(transfer: Json<Transfer>) -> String {
    println!("{}", transfer.0);
    "yo".to_string()
}