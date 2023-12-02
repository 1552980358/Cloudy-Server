use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[path = "account-token/account-token-find-account.rs"]
mod find_account;
pub use find_account::FindAccount;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountToken {

    #[serde(rename = "_id", serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,

    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub account: String,

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

}