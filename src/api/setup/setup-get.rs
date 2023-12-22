use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

use crate::api::setup::SetupEnvironment;
use crate::environment::Environment;
use crate::mongodb::{MongoDB, collection::account::FindOwner};
use crate::mongodb::collection::account::AccountCollection;

#[derive(Serialize, Debug)]
pub struct SetupResponse {
    has_owner: bool,
}

#[get("/?<secret>")]
pub async fn get(
    environment: &State<Environment>, mongodb: &State<MongoDB>, secret: String
) -> Result<Json<SetupResponse>, Status> {
    if !environment.setup_secret_validation(secret) {
        return Err(Status::Unauthorized)
    }

    let has_owner = mongodb.account()
        .has_owner()
        .await;
    let setup_response = SetupResponse { has_owner };
    Ok(Json(setup_response))
}