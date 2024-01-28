use jsonwebtoken::{DecodingKey, Validation};

use crate::jwt::JWTClaims;
use crate::jwt::JWT;

type Result = jsonwebtoken::errors::Result<JWTClaims>;

impl JWT {

    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    fn validation(&self) -> Validation {
        Validation::new(self.algorithm)
    }

    fn decode(&self, jwt: String, validation: Validation) -> Result {
        let decoding_key = self.decoding_key();
        jsonwebtoken::decode(&jwt, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

    pub fn decode_default(&self, jwt: String) -> Result {
        let validation = self.validation();
        self.decode(jwt, validation)
    }

    pub fn decode_ignore_expiry(&self, jwt: String) -> Result {
        let mut validation = self.validation();
        validation.validate_exp = false;
        self.decode(jwt, validation)
    }

}