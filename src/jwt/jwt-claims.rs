use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTClaims {
    pub tok: String,
    pub iat: usize,
    pub exp: usize
}

impl JWTClaims {

    pub fn new(
        token_id: String, issue_at: usize, expire_at: usize
    ) -> Self {
        Self {
            tok: token_id,
            iat: issue_at,
            exp: expire_at
        }
    }

}