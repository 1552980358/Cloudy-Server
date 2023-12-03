use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

use crate::jwt::JWT;
use crate::mongodb::{Mongodb, collections::account::FindOwner};

#[derive(Serialize, Debug)]
struct SetupResponse {
    has_owner: bool,
}

#[get("/?<secret>")]
pub async fn get(
    jwt: &State<JWT>, mongodb: &State<Mongodb>, secret: String
) -> Result<Json<SetupResponse>, Status> {
    if jwt.secret != secret {
        return Err(Status::Unauthorized);
    }

    let has_owner = mongodb.account.find_owner().await;
    let setup_response = SetupResponse {
        has_owner
    };
    Ok(Json(setup_response))
}

