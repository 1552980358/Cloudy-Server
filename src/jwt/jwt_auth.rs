use rocket::http::Status;
use Rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use crate::jwt::jwt_authorization_header::JWTAuthorizationHeader;
use crate::jwt::JWTState;
use crate::mongodb::collection::account_token::{AccountTokenCollection, FindAccount};
use crate::mongodb::MongoDBState;

pub struct Auth(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for Auth {

    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Ok(Some(authorization)) = request.jwt_authorization_header() else {
            return Outcome::Error((Status::BadRequest, ()))
        };

        // Preparation for verifying
        let Some(jwt) = request.jwt() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };
        let Some(mongodb) = request.mongodb() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Ok(jwt_claims) = jwt.decode_default(authorization) else {
            return Outcome::Error((Status::Forbidden, ()))
        };

        // Verify from database
        mongodb.account_token()
            .find_account_id(&jwt_claims.tok, &jwt_claims.iat, &jwt_claims.exp)
            .await
            .map(|account| account.map(|account| Auth(account)))
            .ok()
            .flatten()
            .or_error((Status::Unauthorized, ()))
    }

}