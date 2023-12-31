use jsonwebtoken::{Algorithm};

#[path = "jwt/jwt-environment.rs"]
mod environment;

#[path = "jwt/jwt-setup.rs"]
mod setup;

#[path = "jwt/jwt-claims.rs"]
mod claims;

#[path = "jwt/jwt-encode.rs"]
mod encode;
#[path = "jwt/jwt-decode.rs"]
mod decode;

#[path = "jwt/jwt-auth.rs"]
mod auth;
pub use auth::Auth;

#[path = "jwt/jwt-state.rs"]
mod state;
pub use state::JWTState;

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