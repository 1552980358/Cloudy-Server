use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {

    pub _id: ObjectId,

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