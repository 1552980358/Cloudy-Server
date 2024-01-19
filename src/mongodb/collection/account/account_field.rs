const FIELD_ID: &str = "_id";
const FIELD_USERNAME: &str = "username";
const FIELD_EMAIL: &str = "email";
const FIELD_PASSWORD: &str = "password";
const FIELD_ROLE: &str = "role";
const FIELD_AVATAR: &str = "avatar";

pub struct Field;

impl Field {

    pub fn id<'a>() -> &'a str {
        FIELD_ID
    }

    pub fn username<'a>() -> &'a str {
        FIELD_USERNAME
    }

    pub fn email<'a>() -> &'a str {
        FIELD_EMAIL
    }

    pub fn password<'a>() -> &'a str {
        FIELD_PASSWORD
    }

    pub fn role<'a>() -> &'a str {
        FIELD_ROLE
    }

    pub fn avatar<'a>() -> &'a str {
        FIELD_AVATAR
    }

}