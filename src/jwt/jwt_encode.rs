use jsonwebtoken::{Algorithm, EncodingKey, Header};

use crate::jwt::JWT;
use crate::jwt::Claims;

type JWTResult = jsonwebtoken::errors::Result<String>;

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
        issue_at: u64,
        expire_at: u64
    ) -> JWTResult {
        let header = self.header();
        let encoding_key = self.encoding_key();
        let claims = Claims::new(token_id, issue_at, expire_at);

        jsonwebtoken::encode(&header, &claims, &encoding_key)
    }

}