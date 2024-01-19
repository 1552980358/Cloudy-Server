use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneOptions;
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use Rocket::http::Status;
use Rocket::serde::json::Json;
use Rocket::State;
use serde::{Deserialize, Serialize};

use crate::mongodb::collection::Account;
use crate::mongodb::collection::account::{
    Field as AccountField,
    Role
};

use crate::mongodb::MongoDB;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMetadata {

    // Deserialize from '_id'
    // Serialize as 'id'
    #[serde(alias = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

    pub username: String,
    pub email: String,

    pub nickname: String,

    pub role: Role

}

#[get("/<account_id>")]
pub async fn get_account_metadata(
    mongodb: &State<MongoDB>, account_id: &str
) -> Result<Json<AccountMetadata>, Status> {
    let object_id = ObjectId::parse_str(account_id)
        .map_err(|_| Status::BadRequest)?;

    let filter = doc! {
        AccountField::id(): object_id,
    };
    let projection = doc! {
        // Hide password field
        AccountField::password(): 0,
        // Hide avatar field
        AccountField::avatar(): 0,
    };
    let find_one_options = FindOneOptions::builder()
        .projection(projection)
        .build();

    mongodb.view::<Account, AccountMetadata>()
        .find_one(filter, find_one_options)
        .await
        .map_err(|_| { Status::InternalServerError })?
        .map(|account_metadata| Json(account_metadata))
        .ok_or_else(|| Status::NotFound)
}