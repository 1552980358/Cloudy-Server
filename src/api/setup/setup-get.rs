use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

use crate::jwt::JWT;
use crate::mongodb::{MongoDB, collection::account::FindOwner};
use crate::mongodb::collection::account::AccountCollection;

#[derive(Serialize, Debug)]
pub struct SetupResponse {
    has_owner: bool,
}

#[get("/?<secret>")]
pub async fn get(
    jwt: &State<JWT>, mongodb: &State<MongoDB>, secret: String
) -> Result<Json<SetupResponse>, Status> {
    if jwt.secret != secret {
        return Err(Status::Unauthorized);
    }

    let has_owner = mongodb.account().find_owner().await;
    let setup_response = SetupResponse {
        has_owner
    };
    Ok(Json(setup_response))
}