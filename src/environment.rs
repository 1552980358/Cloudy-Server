use std::collections::HashMap;

#[path = "environment/environment-setup.rs"]
mod setup;

pub struct Environment {
    variables: HashMap<String, String>,
}

const ENVIRONMENT_DIVIDER: char = '_';

impl Environment {

    fn divider() -> char {
        ENVIRONMENT_DIVIDER
    }

    fn key(scheme: &str, field: &str) -> String {
        // [scheme], [field] -> "<SCHEME>_<FIELD>"
        format!("{}{}{}", scheme, Environment::divider(), field)
    }

    pub fn variable(&self, scheme: &str, field: &str) -> Option<String> {
        let key = Environment::key(scheme, field);
        self.variables.get(&key)
            .map(|variable| variable.to_owned())
    }

}