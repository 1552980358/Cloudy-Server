use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id,
    deserialize_hex_string_from_object_id
};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::mongodb::object_id::object_new_hex;
use crate::mongodb::collection::Collection as MongoDBCollection;
use crate::mongodb::MongoDB;

#[path = "account/account-login.rs"]
mod login;
pub use login::Login;

#[path = "account/account-find-owner.rs"]
mod find_owner;
pub use find_owner::FindOwner;

#[path = "account/account-field.rs"]
pub mod field;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {

    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    #[serde(default = "object_new_hex")]
    pub id: String,

    pub username: String,
    pub email: String,
    pub password: String,

    pub nickname: String,

    pub role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {

    #[serde(rename = "owner")]
    Owner,

    #[serde(rename = "admin")]
    Administrator,

    #[serde(rename = "user")]
    User

}

pub trait AccountCollection {
    fn account(&self) -> Collection<Account>;
}

impl AccountCollection for MongoDB {
    fn account(&self) -> Collection<Account> {
        self.collection()
    }
}

const COLLECTION_ACCOUNT: &str = "account";
impl MongoDBCollection for Account {
    fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT
    }
}