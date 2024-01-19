use mongodb::bson::doc;
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use mongodb::options::FindOneOptions;
use Rocket::http::Status;
use Rocket::response::Redirect;
use Rocket::State;
use serde::Deserialize;

use crate::mongodb::collection::account::{
    Account,
    Field as AccountField
};

use crate::mongodb::{Filter, MongoDB};

#[derive(Deserialize, Debug)]
struct FindAccountView {
    #[serde(alias = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String
}

#[get("/find/<identity>?<allow_blank_response>")]
pub async fn find_account(
    mongodb: &State<MongoDB>,
    identity: &str,
    allow_blank_response: Option<bool>
) -> Result<Redirect, Status> {
    let allow_blank_response = allow_blank_response.unwrap_or_else(|| false);

    let filter = doc! {
        Filter::or(): [
            { AccountField::username(): identity },
            { AccountField::email(): identity }
        ]
    };
    let projection = doc! {
        AccountField::id(): 1
    };
    let find_one_options = FindOneOptions::builder()
        .projection(projection)
        .build();

    mongodb.view::<Account, FindAccountView>()
        .find_one(filter, find_one_options)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|find_account_view| {
            Redirect::to(format!("/account/{}", find_account_view.id))
        })
        .ok_or_else(|| {
            if allow_blank_response { Status::Ok }
            else { Status::NotFound }
        })
}