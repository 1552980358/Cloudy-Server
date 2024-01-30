use Rocket::http::Status;
use Rocket::outcome::IntoOutcome;
use Rocket::{Request, State};
use Rocket::request::{FromRequest, Outcome};

use crate::jwt::{Claims, JWT, JWTAuthorizationHeader, JWTState};
use crate::mongodb::collection::account_token::{AccountTokenCollection, FindAccount, Register};
use crate::mongodb::{MongoDB, MongoDBState};
use crate::util::time::system_time_secs;

pub struct Auth {
    pub account_id: String,
    pub token_id: String,
    pub token_issue: u64,
    pub token_expiry: u64,
    pub current_time: u64,
}

const RENEW_MAX_TIMEOUT: u64 = 7 * 24 * 60 * 60;
#[get("/")]
pub async fn renew_token(jwt: &State<JWT>, mongodb: &State<MongoDB>, auth: Auth) -> Result<String, Status> {
    // Update to database
    let renewal_expiry = auth.current_time + RENEW_MAX_TIMEOUT;
    mongodb.account_token()
        .register_renewal(&auth.account_id, &auth.token_id, &auth.token_issue, &renewal_expiry)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|account_token| {
            // Encode to jwt and return result
            jwt.encode(account_token.id, account_token.issue, account_token.expiry)
                .map_err(|_| Status::InternalServerError)
        })
        // None => renew: false
        .ok_or_else(|| Status::NotAcceptable)?
}

impl Auth {
    pub fn new(
        account_id: String,
        claims: Claims,
        current_time: u64
    ) -> Self {
        Self {
            account_id,
            token_id: claims.tok,
            token_issue: claims.iat,
            token_expiry: claims.exp,
            current_time
        }
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for Auth {

    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let Ok(Some(authorization)) = request.jwt_authorization_header() else {
            return Outcome::Error((Status::BadRequest, ()))
        };

        // Get all required State
        let (Some(jwt), Some(mongodb)) = (request.jwt(), request.mongodb()) else {
            return Outcome::Error((Status::InternalServerError, ()))
        };
        let Ok(current_time) = system_time_secs() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Ok(claims) = jwt.decode_ignore_expiry(authorization) else {
            return Outcome::Error((Status::Unauthorized, ()))
        };

        if !(claims.exp .. claims.exp + RENEW_MAX_TIMEOUT).contains(&current_time) {
            return Outcome::Error((Status::Forbidden, ()))
        }

        let account_id_with_renewal = mongodb.account_token()
            .find_account_id_with_renewal(&claims.tok, &claims.iat, &claims.exp)
            .await;
        // Check if Err
        let Ok(account_id_with_renewal) = account_id_with_renewal else {
            return Outcome::Error((Status::InternalServerError, ()))
        };
        // Check if None
        let Some((account_id, renewal)) = account_id_with_renewal else {
            return Outcome::Error((Status::Unauthorized, ()))
        };

        // Do a `renewal` field checking
        renewal.then(|| Auth::new(account_id, claims, current_time))
            .or_error((Status::NotAcceptable, ()))
    }

}