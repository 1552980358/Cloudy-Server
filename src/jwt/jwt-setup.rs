use std::str::FromStr;
use jsonwebtoken::Algorithm;

use crate::environment::Environment;

use crate::jwt::JWT;
use crate::jwt::environment::JWTEnvironment;

impl JWT {

    pub fn setup(environment: &Environment) -> Self {
        let secret = environment.jwt_secret();
        let algorithm = jwt_algorithm(environment.jwt_algorithm());

        JWT::new(secret, algorithm)
    }

}

fn jwt_algorithm(algorithm: Option<String>) -> Algorithm {
    match algorithm {
        Some(algorithm) => {
            Algorithm::from_str(algorithm.as_str())
                .unwrap_or(Algorithm::default())
        }
        None => { Algorithm::default() }
    }
}