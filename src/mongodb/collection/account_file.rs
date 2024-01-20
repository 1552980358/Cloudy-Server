use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id,
    deserialize_hex_string_from_object_id
};
use serde::{Deserialize, Serialize};

use crate::mongodb::collection::Collection as MongoDBCollection;
use crate::mongodb::object_id::{
    serialize_option_hex_string_to_object_id,
    deserialize_option_hex_string_from_object_id
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountFile {

    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,

    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub account: String,

    pub filename: String,

    pub filetype: Filetype,

    #[serde(serialize_with = "serialize_option_hex_string_to_object_id")]
    #[serde(deserialize_with = "deserialize_option_hex_string_from_object_id")]
    pub directory: Option<String>,

    pub size: usize,

    pub md5: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Filetype {

    #[serde(rename = "image")]
    Image(String),

    #[serde(rename = "audio")]
    Audio(String),

    #[serde(rename = "video")]
    Video(String),

    #[serde(rename = "application")]
    Application(String),

    #[serde(rename = "text")]
    Text(String),

    #[serde(rename = "unknown")]
    Unknown

}

const COLLECTION_ACCOUNT_DIRECTORY: &str = "account-directory";
impl MongoDBCollection for AccountFile {
    fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT_DIRECTORY
    }
}