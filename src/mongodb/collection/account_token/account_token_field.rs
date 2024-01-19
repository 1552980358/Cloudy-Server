const FIELD_ID: &str = "_id";
const FIELD_ISSUE: &str = "issue";
const FIELD_EXPIRY: &str = "expiry";

pub struct Field;

impl Field {

    pub fn id<'a>() -> &'a str {
        FIELD_ID
    }

    pub fn issue<'a>() -> &'a str {
        FIELD_ISSUE
    }

    pub fn expiry<'a>() -> &'a str {
        FIELD_EXPIRY
    }

}