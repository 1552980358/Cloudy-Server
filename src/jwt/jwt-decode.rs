use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::Error as JWTError;

use crate::jwt::claims::JWTClaims;
use crate::jwt::JWT;

impl JWT {

    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    fn validation(&self) -> Validation {
        match () {
            _ if self.algorithm == Algorithm::default() => {
                Validation::default()
            }
            _ => {
                Validation::new(self.algorithm)
            }
        }
    }

    pub fn decode(
        &self,
        jwt: String
    ) -> Result<TokenData<JWTClaims>, JWTError> {
        let decoding_key = self.decoding_key();
        let validation = self.validation();

        jsonwebtoken::decode(
            &jwt, &decoding_key, &validation
        )
    }

}