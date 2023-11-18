use std::fmt::Display;
use jsonwebtoken::{Algorithm};

#[path = "jwt/jwt-env.rs"]
mod env;

#[path = "jwt/jwt-setup.rs"]
mod setup;

#[path = "jwt/jwt-claims.rs"]
mod claims;

#[path = "jwt/jwt-encode.rs"]
mod encode;
#[path = "jwt/jwt-decode.rs"]
mod decode;

const PANIC: &str = "JWT Panic: ";
fn self_panic<M: Display>(message: M) -> ! {
    panic!("{}{}", PANIC, message)
}

pub struct JWT {
    secret: String,
    algorithm: Algorithm,
}

impl JWT {

    fn new(secret: String, algorithm: Algorithm) -> Self {
        Self { secret, algorithm }
    }

}