#[macro_use]
extern crate rocket;

use dynomite::dynamodb::DynamoDbClient;
use rocket::http::Status;
use rocket::Build;
use rusoto_core::Region;

mod users;
pub mod error_response;

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    let db = DynamoDbClient::new(Region::default());
    rocket::build().mount("/", routes![])
}
