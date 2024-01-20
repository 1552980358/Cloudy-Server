use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id,
    deserialize_hex_string_from_object_id
};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::mongodb::object_id::{
    serialize_option_hex_string_to_object_id,
    deserialize_option_hex_string_from_object_id
};
use crate::mongodb::collection::Collection as MongoDBCollection;
use crate::mongodb::MongoDB;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDirectory {

    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub account: String,

    pub name: String,

    #[serde(serialize_with = "serialize_option_hex_string_to_object_id")]
    #[serde(deserialize_with = "deserialize_option_hex_string_from_object_id")]
    pub parent: Option<String>,

}

pub trait AccountDirectoryCollection {
    fn account_directory(&self) -> Collection<AccountDirectory>;
}

impl AccountDirectoryCollection for MongoDB {
    fn account_directory(&self) -> Collection<AccountDirectory> {
        self.collection()
    }
}

const COLLECTION_ACCOUNT_DIRECTORY: &str = "account-directory";
impl MongoDBCollection for AccountDirectory {
    fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT_DIRECTORY
    }
}