use std::time::{SystemTime, UNIX_EPOCH};
use rocket::State;
use rocket::http::Status;
use serde::{Deserialize};

use crate::jwt::JWT;
use crate::mongodb::{MongoDB, collection::account::Login };
use crate::mongodb::collection::{Account, AccountToken};
use crate::mongodb::collection::account::AccountCollection;
use crate::mongodb::collection::account_token::{AccountTokenCollection, Register};

#[path = "auth-post-request.rs"]
mod request;

#[derive(Deserialize, Debug)]
pub struct AuthRequestBody {
    pub username: String,
    pub password: String,
}

const DURATION_DEFAULT: usize = 604800000usize;

#[post("/?<duration>", data = "<auth_request_body>")]
pub async fn post(
    mongodb: &State<MongoDB>,
    jwt: &State<JWT>,
    duration: Option<usize>,
    auth_request_body: AuthRequestBody
) -> Result<String, Status> {
    // Request account information
    let Some(account) = login(mongodb, auth_request_body).await else {
        return Err(Status::Unauthorized);
    };

    // Prepare for register credential
    // Issue time
    let Some(issue) = time_millis() else {
        return Err(Status::InternalServerError);
    };
    let duration = duration.unwrap_or_else(|| DURATION_DEFAULT);

    // Register and return
    let Some(account_token) = register_token(mongodb, account, issue, duration).await else {
        return Err(Status::InternalServerError);
    };
    jwt_encode(jwt, account_token)
        .map(|jwt| Ok(jwt))
        .unwrap_or_else(|| Err(Status::InternalServerError))
}

async fn login(mongodb: &MongoDB, auth_request_body: AuthRequestBody) -> Option<Account> {
    mongodb.account()
        .login(auth_request_body.username, auth_request_body.password)
        .await
        .ok()
        .flatten()
}

fn time_millis() -> Option<usize> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map(|time_millis| time_millis as usize)
        .ok()
}

async fn register_token(
    mongodb: &MongoDB, account: Account, issue: usize, duration: usize
) -> Option<AccountToken> {
    mongodb.account_token()
        .register(account, issue, duration)
        .await
}

fn jwt_encode(jwt: &JWT, account_token: AccountToken) -> Option<String> {
    jwt.encode(account_token.id, account_token.issue, account_token.expiry)
        .ok()
}