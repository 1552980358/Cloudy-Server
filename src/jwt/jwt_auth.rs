use regex::Regex;
use rocket::http::Status;
use Rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use crate::jwt::JWTState;
use crate::mongodb::collection::account_token::{AccountTokenCollection, FindAccount};
use crate::mongodb::MongoDBState;
use crate::rocket::RequestHeader;

pub struct Auth(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for Auth {

    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_jwt = request.authorization()
            .map(|authorization| authorization_jwt(authorization).ok())
            .flatten()
            .flatten();

        let Some(authorization_jwt) = authorization_jwt else {
            return Outcome::Error((Status::BadRequest, ()))
        };

        // Preparation for verifying
        let Some(jwt) = request.jwt() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };
        let Some(mongodb) = request.mongodb() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Ok(jwt_claims) = jwt.decode(authorization_jwt) else {
            return Outcome::Error((Status::Unauthorized, ()))
        };

        // Verify from database
        mongodb.account_token()
            .find_account(jwt_claims.tok, jwt_claims.iat, jwt_claims.exp)
            .await
            .map(|account| account.map(|account| Auth(account)))
            .ok()
            .flatten()
            .or_error((Status::Unauthorized, ()))
    }

}


const AUTHORIZATION_REGEX: &str = r"JWT (?<jwt>^([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_\-+/=]*))";
const AUTHORIZATION_REGEX_JWT: &str = "jwt";
fn authorization_jwt(authorization: String) -> Result<Option<String>, regex::Error> {
    let regex = Regex::new(AUTHORIZATION_REGEX)?;
    let Some(captures) = regex.captures(&authorization) else {
        return Ok(None)
    };
    let jwt = captures.name(AUTHORIZATION_REGEX_JWT)
        .map(|jwt_match| jwt_match.as_str().to_string());
    Ok(jwt)
}