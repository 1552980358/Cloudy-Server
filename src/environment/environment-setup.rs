use std::collections::HashMap;

use crate::environment::Environment;

const ENVIRONMENT_PREFIX: &str = "CLOUDY_";

impl Environment {

    fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn setup() -> Self {
        // Collect and filter
        let environment_variables = std::env::vars()
            .collect::<Vec<(String, String)>>();
        let cloudy_variables = environment_variables.into_iter()
            // Filter all variables starting with `CLOUDY_`
            .filter(|(key, value)| key.starts_with(ENVIRONMENT_PREFIX))
            .collect::<Vec<(String, String)>>();

        // Length of prefix
        let prefix_length = ENVIRONMENT_PREFIX.len();
        // Set to Environment
        let mut environment = Environment::new();
        for variable in cloudy_variables {
            // "CLOUDY_SCHEME_FIELD" -> "SCHEME_FIELD"
            let key = variable.0[prefix_length..].to_string();
            let value = variable.1;
            environment.variables.insert(key, value);
        }

        // Return
        environment
    }

}