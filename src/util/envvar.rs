use std::env::VarError;

pub fn with_default(key: &str, default: String) -> String {
    with_result(key).unwrap_or_else(|_| default)
}

pub fn with_result(key: &str) -> Result<String, VarError> {
    return std::env::var(key)
}

pub fn with_option(key: &str) -> Option<String> {
    match with_result(key) {
        Ok(value) => { Some(value) }
        Err(_) => { None }
    }
}