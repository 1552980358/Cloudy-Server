use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id,
    deserialize_hex_string_from_object_id
};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[path = "account-token/account-token-register.rs"]
mod register;

#[path = "account-token/account-token-field.rs"]
pub mod field;

pub use register::Register;

use crate::mongodb::{
    collection::Collection as MongoDBCollection,
    collection::Account,
    MongoDB
};

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

impl AccountToken {

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

pub trait AccountTokenCollection {
    fn account_token(&self) -> Collection<AccountToken>;
}
impl AccountTokenCollection for MongoDB {
    fn account_token(&self) -> Collection<AccountToken> {
        self.collection()
    }
}

const COLLECTION_ACCOUNT_TOKEN: &str = "account-token";
impl MongoDBCollection for AccountToken {
    fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT_TOKEN
    }
}