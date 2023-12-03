use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id,
    deserialize_hex_string_from_object_id
};
use serde::{Deserialize, Serialize};

use crate::mongodb::collections::Account;

#[path = "account-token/account-token-find-account.rs"]
mod find_account;
pub use find_account::FindAccount;

#[path = "account-token/account-token-register.rs"]
mod register;
pub use register::Register;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountToken {

    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
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

    pub fn new(account: Account, issue: usize, duration: usize) -> Self {
        let id = ObjectId::new().to_hex();
        let account = account.id;
        let expiry = issue + duration;

        Self {
            id,
            account,
            issue,
            duration,
            expiry,
            valid: true
        }
    }

}