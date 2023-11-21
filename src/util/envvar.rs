use std::env::VarError;

pub fn with_result(key: &str) -> Result<String, VarError> {
    std::env::var(key)
}

pub fn with_option(key: &str) -> Option<String> {
    with_result(key).ok()
}