pub struct Operator;

const LOGICAL_QUERY_OR: &str = "$or";
impl Operator {
    pub fn logical_query_or<'a>() -> &'a str {
        LOGICAL_QUERY_OR
    }
}

const FIELD_UPDATE_SET: &str = "$set";
impl Operator {
    pub fn field_update_set<'a>() -> &'a str {
        FIELD_UPDATE_SET
    }
}