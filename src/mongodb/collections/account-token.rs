use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountToken {

    pub _id: ObjectId,

    pub account: ObjectId,

    pub issue: usize,
    pub duration: usize,
    pub expiry: usize,

    pub valid: bool,

}

const COLLECTION_ACCOUNT_TOKEN: &str = "account-token";

impl AccountToken {

    pub fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT_TOKEN
    }

    pub fn account(&self) -> String {
        self.account.to_hex()
    }

}