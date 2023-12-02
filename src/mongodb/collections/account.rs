use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {

    #[serde(rename = "_id", serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,

    pub username: String,
    pub password: String,

    pub nickname: String,

    pub role: Role,

}

const COLLECTION_ACCOUNT: &str = "account";

impl Account {

    pub fn name<'a>() -> &'a str {
        COLLECTION_ACCOUNT
    }

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