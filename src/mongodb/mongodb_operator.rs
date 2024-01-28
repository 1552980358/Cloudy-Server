pub struct Operator;

const LOGICAL_QUERY_OR: &str = "$or";
impl Operator {
    pub fn logical_query_or<'a>() -> &'a str {
        LOGICAL_QUERY_OR
    }
}