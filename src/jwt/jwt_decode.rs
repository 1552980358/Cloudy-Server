use jsonwebtoken::{DecodingKey, Validation};

use crate::jwt::JWTClaims;
use crate::jwt::JWT;

type Result<R> = jsonwebtoken::errors::Result<R>;

impl JWT {

    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    fn validation(&self) -> Validation {
        Validation::new(self.algorithm)
    }

    pub fn decode(
        &self,
        jwt: String
    ) -> Result<JWTClaims> {
        let decoding_key = self.decoding_key();
        let validation = self.validation();

        jsonwebtoken::decode(&jwt, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

}