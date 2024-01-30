use Rocket::data::{FromData, Outcome};
use rocket::State;
use rocket::http::Status;
use Rocket::{Data, Request};
use serde::{Deserialize};

use crate::jwt::JWT;
use crate::mongodb::MongoDB;
use crate::mongodb::collection::account::{
    Account,
    AccountCollection,
    Login
};
use crate::mongodb::collection::account_token::{
    AccountTokenCollection,
    Register
};
use crate::rocket::{PostData, RequestHeader};
use crate::util::time::system_time_secs;

#[derive(Deserialize, Debug)]
pub struct AuthRequestBody {
    pub username: String,
    pub password: String,
}

type AuthRequestBodyDecodeResult = serde_json::error::Result<AuthRequestBody>;
impl AuthRequestBody {
    fn decode(request_body: String) -> AuthRequestBodyDecodeResult {
        serde_json::from_str(&*request_body)
    }
}

#[derive(Debug)]
pub enum AuthRequestError {
    InvalidContentType,
    ContentLengthMissing,
    InvalidContent,
    InvalidJSONContent
}

#[async_trait]
impl<'r> FromData<'r> for AuthRequestBody {

    type Error = AuthRequestError;

    async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        if !request.is_json_content() {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::InvalidContentType)
            )
        }

        let Some(content_length) = request.content_length() else {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::ContentLengthMissing)
            )
        };

        let Ok(request_body) = data.string(content_length).await else {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::InvalidContent)
            )
        };

        AuthRequestBody::decode(request_body)
            .map(|auth_request_body| {
                Outcome::Success(auth_request_body)
            })
            .unwrap_or_else(|_| {
                Outcome::Error(
                    (Status::BadRequest, AuthRequestError::InvalidJSONContent)
                )
            })
    }

}

const DURATION_DEFAULT: u64 = 604800;

#[post("/?<duration>&<disposable>", data = "<auth_request_body>")]
pub async fn login_auth(
    mongodb: &State<MongoDB>,
    jwt: &State<JWT>,
    duration: Option<u64>,
    disposable: Option<bool>,
    auth_request_body: AuthRequestBody
) -> Result<String, Status> {
    // Request account information
    let Some(account) = login(mongodb, auth_request_body).await else {
        return Err(Status::Unauthorized);
    };

    // Prepare for register credential
    // Issue time
    let Ok(issue) = system_time_secs() else {
        return Err(Status::InternalServerError);
    };
    let duration = duration.unwrap_or_else(|| DURATION_DEFAULT);
    // If not specified, allow to renewal
    let renewal = !disposable.unwrap_or_else(|| false);
    let register_token = mongodb.account_token()
        .register_new(&account.id, &issue, &duration, &renewal)
        .await;

    // Register and return
    let Ok(account_token) = register_token else {
        return Err(Status::InternalServerError);
    };
    jwt.encode(account_token.id, account_token.issue, account_token.expiry)
        .map_err(|_| Status::InternalServerError)
}

async fn login(mongodb: &MongoDB, auth_request_body: AuthRequestBody) -> Option<Account> {
    mongodb.account()
        .login(auth_request_body.username, auth_request_body.password)
        .await
        .ok()
        .flatten()
}