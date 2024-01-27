use jsonwebtoken::{Algorithm};

mod jwt_environment;
use jwt_environment::JWTEnvironment;

mod jwt_setup;

mod jwt_claims;
use jwt_claims::JWTClaims;

mod jwt_encode;

mod jwt_decode;

mod jwt_auth;
pub use jwt_auth::Auth;

mod jwt_state;
pub use jwt_state::JWTState;

mod jwt_authorization_header;
pub use jwt_authorization_header::JWTAuthorizationHeader;

macro_rules! jwt_panic {
    ($display:expr) => {
        panic!("JWT Panic: {}", $display)
    };
    ($($arg:tt)*) => {
        panic!("JWT Panic: {}", format!($($arg)*))
    };
}
use jwt_panic;

pub struct JWT {
    secret: String,
    algorithm: Algorithm,
}

impl JWT {

    fn new(secret: String, algorithm: Algorithm) -> Self {
        Self { secret, algorithm }
    }

}