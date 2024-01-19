use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use Rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use crate::jwt::JWTState;
use crate::mongodb::collection::account_token::{AccountTokenCollection, FindAccount};
use crate::mongodb::MongoDBState;
use crate::rocket::RequestHeader;

pub struct Auth(pub String);

#[derive(Debug)]
pub enum AuthError {
    MissingAuthorization,
    InvalidFormat,
    Expired,
    InternalError,
    Unauthorized
}

#[async_trait]
impl<'r> FromRequest<'r> for Auth {

    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check if exists and format
        let Some(authorization) = request.authorization() else {
            return Outcome::Error(
                (Status::BadRequest, AuthError::MissingAuthorization)
            )
        };
        // Authorization: JWT <JWT_VALUE>
        let Some(authorization) = extract_jwt(authorization) else {
            return Outcome::Error(
                (Status::BadRequest, AuthError::InvalidFormat)
            )
        };

        // Verify
        let Some(jwt) = request.jwt() else {
            return Outcome::Error(
                (Status::InternalServerError, AuthError::InternalError)
            )
        };

        let jwt_claims = match jwt.decode(authorization) {
            Ok(jwt_claims) => { jwt_claims }
            Err(error) => {
                let auth_error = match error.kind() {
                    ErrorKind::ExpiredSignature => { AuthError::Expired }
                    _ => { AuthError::InvalidFormat }
                };
                return Outcome::Error((Status::Unauthorized, auth_error))
            }
        };

        let Some(mongodb) = request.mongodb() else {
            return Outcome::Error(
                (Status::InternalServerError, AuthError::InternalError)
            )
        };

        // Verify from database
        let find_account_result = mongodb.account_token()
            .find_account(jwt_claims.tok, jwt_claims.iat, jwt_claims.exp)
            .await;

        find_account_result.map(|account| {
            account.map(|account| Auth(account))
                .or_error((Status::Unauthorized, AuthError::Unauthorized))
        }).unwrap_or_else(|_| {
            Outcome::Error(
                (Status::Unauthorized, AuthError::InternalError)
            )
        })
    }

}

const AUTHORIZATION_PREFIX: &str = "JWT ";
fn extract_jwt(authorization: String) -> Option<String> {
    if !authorization.starts_with(AUTHORIZATION_PREFIX) {
        return None;
    }
    let authorization_header_len = authorization.len();
    let jwt_prefix_len = AUTHORIZATION_PREFIX.len();
    if authorization_header_len <= jwt_prefix_len {
        return None;
    }
    let authorization_jwt = authorization[jwt_prefix_len .. authorization_header_len]
        .to_string();
    Some(authorization_jwt)
}