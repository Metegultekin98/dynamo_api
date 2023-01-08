use dynomite::dynamodb::{DynamoDbClient, DynamoDb, PutItemInput};
use rocket::{serde::json::Json, State, http::Status};
use uuid::Uuid;

use super::User;

#[post("/", format = "json", data = "<user>")]
pub async fn create(
    db: &State<DynamoDbClient>,
    user: Json<User>,
) -> Result<(Status, Json<User>), (Status, String)> {
    let u = User {
        id: Some(Uuid::new_v4()),
        ..user.0
    };

    db.put_item(PutItemInput {
        table_name: super::table_name(),
        item: u.clone().into(),
        ..PutItemInput::default()
    })
    .await
    .map(|_| (Status::Created, Json(u)))
    .map_err(|e| (Status::InternalServerError, e.to_string()))
}