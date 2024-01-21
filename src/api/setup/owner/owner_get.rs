use Rocket::http::Status;
use Rocket::serde::json::Json;
use Rocket::serde::Serialize;
use Rocket::State;

use crate::mongodb::collection::account::{
    Account,
    AccountCollection,
    FindOwner
};
use crate::mongodb::MongoDB;

#[derive(Serialize, Debug)]
pub struct Response {
    pub id: String,
    pub username: String,
    pub nickname: String,
}

impl Response {
    pub fn from_account(account: Account) -> Self {
        Self {
            id: account.id,
            username: account.username,
            nickname: account.nickname,
        }
    }
}

#[get("/")]
pub async fn check_owner_state(mongodb: &State<MongoDB>) -> Result<Json<Response>, Status> {
    mongodb.account()
        .find_owner()
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|account| Json(Response::from_account(account)))
        .ok_or_else(|| Status::NotFound)
}