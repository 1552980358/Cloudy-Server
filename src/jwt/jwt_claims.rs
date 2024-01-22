use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTClaims {
    pub tok: String,
    pub iat: u64,
    pub exp: u64
}

impl JWTClaims {

    pub fn new(
        token_id: String, issue_at: u64, expire_at: u64
    ) -> Self {
        Self {
            tok: token_id,
            iat: issue_at,
            exp: expire_at
        }
    }

}