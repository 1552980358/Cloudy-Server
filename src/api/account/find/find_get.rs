use mongodb::bson::{doc, to_document};
use mongodb::bson::serde_helpers::deserialize_hex_string_from_object_id;
use mongodb::options::FindOneOptions;
use Rocket::http::Status;
use Rocket::response::Redirect;
use Rocket::State;
use serde::{Deserialize, Serialize};

use crate::mongodb::collection::account::{
    AccountCollection,
    Field as AccountField
};

use crate::mongodb::{Operator, MongoDB};

#[derive(Serialize, Debug)]
struct Projection {
    #[serde(alias = "_id")]
    pub id: i32
}

#[derive(Deserialize, Debug)]
struct DatabaseView {
    #[serde(alias = "_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String
}

#[get("/<identity>?<allow_blank_response>")]
pub async fn find_account(
    mongodb: &State<MongoDB>,
    identity: &str,
    allow_blank_response: Option<bool>
) -> Result<Redirect, Status> {
    let allow_blank_response = allow_blank_response.unwrap_or_else(|| false);

    let filter = doc! {
        Operator::logical_query_or(): vec! [
            doc! { AccountField::username(): identity },
            doc! { AccountField::email(): identity }
        ]
    };
    let projection = Projection { id: 1 };
    let projection_document = to_document(&projection)
        .map_err(|_| Status::InternalServerError)?;
    let find_one_options = FindOneOptions::builder()
        .projection(projection_document)
        .build();

    mongodb.account_view::<DatabaseView>()
        .find_one(filter, find_one_options)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|database_view| {
            Redirect::to(format!("/account/{}", database_view.id))
        })
        .ok_or_else(|| {
            if allow_blank_response { Status::Ok }
            else { Status::NotFound }
        })
}