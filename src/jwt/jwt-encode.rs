use jsonwebtoken::{Algorithm, EncodingKey, Header};
use jsonwebtoken::errors::Error as JWTError;

use crate::jwt::JWT;
use crate::jwt::claims::JWTClaims;

impl JWT {

    fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }

    fn header(&self) -> Header {
        match () {
            _ if self.algorithm == Algorithm::default() => {
                Header::default()
            }
            _ => {
                Header::new(self.algorithm)
            }
        }
    }

    pub fn encode(
        &self,
        token_id: String,
        issue_at: usize,
        expire_at: usize
    ) -> Result<String, JWTError> {
        let header = self.header();
        let encoding_key = self.encoding_key();
        let jwt_claims = JWTClaims::new(token_id, issue_at, expire_at);

        jsonwebtoken::encode(
            &header, &jwt_claims, &encoding_key
        )
    }

}