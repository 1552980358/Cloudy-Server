use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use mongodb::options::FindOneOptions;

use Rocket::http::Status;
use Rocket::serde::{Deserialize, Serialize};
use Rocket::serde::json::Json;
use Rocket::State;

use crate::jwt::Auth;
use crate::mongodb::collection::account::{
    Account, field as AccountField, Role
};
use crate::mongodb::MongoDB;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponseBody {

    // Deserialize from '_id'
    // Serialize as 'id'
    #[serde(alias = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

    pub username: String,

    pub nickname: String,

    pub role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>

}

#[get("/")]
pub async fn get(
    mongodb: &State<MongoDB>,
    auth: Auth
) -> Result<Json<AccountResponseBody>, Status> {
    let object_id = ObjectId::parse_str(auth.0)
        .map_err(|_| Status::InternalServerError)?;

    let filter = doc! {
        AccountField::id(): object_id,
    };
    let projection = doc! {
        // Hide password field
        AccountField::password(): 0
    };
    let find_one_options = FindOneOptions::builder()
        .projection(projection)
        .build();

    mongodb.view::<Account, AccountResponseBody>()
        .find_one(filter, find_one_options)
        .await
        .map_err(|_| { Status::InternalServerError })?
        .map(|account_response_body| Json(account_response_body))
        .ok_or_else(|| Status::NotFound)
}