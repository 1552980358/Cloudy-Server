use std::collections::HashMap;

mod environment_setup;

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

    fn variable(&self, key: &String) -> Option<String> {
        self.variables.get(key)
            .map(|variable| variable.to_owned())
    }

    /**
     * Read variable and return variable option
     **/
    pub fn read_variable(&self, scheme: &str, field: &str) -> Option<String> {
        let key = Environment::key(scheme, field);
        self.variable(&key)
            .map(|variable| variable.to_owned())
    }

    /**
     * Read variable, remove variable from [Environment.variables] if exists,
     * and return variable option
     **/
    pub fn take_variable(&mut self, scheme: &str, field: &str) -> Option<String> {
        let key = Environment::key(scheme, field);
        let variable = self.variable(&key);
        if variable.is_some() {
            self.variables
                .remove_entry(&key);
        }
        variable
    }

}