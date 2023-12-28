use mongodb::bson::doc;
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use mongodb::options::FindOneOptions;
use Rocket::http::Status;
use Rocket::response::Redirect;
use Rocket::State;
use serde::{Deserialize};

use crate::mongodb::collection::Account;
use crate::mongodb::collection::account::field as AccountField;
use crate::mongodb::MongoDB;

#[derive(Deserialize, Debug)]
pub struct AccountID {
    #[serde(alias = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String
}

#[get("/username/<account_username>")]
pub async fn get(
    mongodb: &State<MongoDB>, account_username: &str
) -> Result<Redirect, Status> {
    let filter = doc! {
        AccountField::username(): account_username
    };
    let projection = doc! {
        AccountField::id(): 1
    };
    let find_one_options = FindOneOptions::builder()
        .projection(projection)
        .build();

    mongodb.view::<Account, AccountID>()
        .find_one(filter, find_one_options)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|account_id| Redirect::to(format!("/account/{}", account_id.id)))
        .ok_or_else(|| Status::NotFound)
}