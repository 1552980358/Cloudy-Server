use std::str::FromStr;
use jsonwebtoken::Algorithm;

use crate::jwt::JWT;
use crate::jwt::env;

impl JWT {

    pub async fn setup() -> Self {
        let secret = env::secret();

        let algorithm = env::algorithm();
        let algorithm = jwt_algorithm(algorithm);

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