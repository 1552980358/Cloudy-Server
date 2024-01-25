use std::str::FromStr;
use jsonwebtoken::Algorithm;

use crate::environment::Environment;

use crate::jwt::{
    JWT,
    JWTEnvironment
};

impl JWT {

    pub fn setup(environment: &mut Environment) -> Self {
        let secret = environment.jwt_secret();
        let algorithm = environment.jwt_algorithm()
            .map(|algorithm| Algorithm::from_str(&*algorithm).ok())
            .flatten()
            .unwrap_or_default();

        JWT::new(secret, algorithm)
    }

}