use std::collections::HashMap;

#[path = "environment/environment-setup.rs"]
mod setup;

pub struct Environment {
    variables: HashMap<String, String>,
}

const ENVIRONMENT_DIVIDER: &str = "_";
const ENVIRONMENT_PREFIX: &str = "CLOUDY_";

impl Environment {

    fn divider<'a>() -> &'a str {
        ENVIRONMENT_DIVIDER
    }

    fn key(scheme: &str, field: &str) -> String {
        /**
         * [scheme], [field] ->
         * "<SCHEME>_<FIELD>"
         **/
        format!("{}{}{}", scheme, Environment::divider(), field)
    }

    pub fn exists(&self, scheme: &str, field: &str) -> bool {
        let key = Environment::key(scheme, field);
        self.variables.contains_key(&key)
    }

    pub fn variable(&self, scheme: &str, field: &str) -> Option<String> {
        let key = Environment::key(scheme, field);
        self.variables.get(&key)
            .map(|variable| variable.to_owned())
    }

}