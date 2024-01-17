use crate::environment::Environment;

const ENVIRONMENT_SETUP: &str = "SETUP";
const ENVIRONMENT_SETUP_SECRET: &str = "SECRET";

pub trait SetupEnvironment {

    fn setup(&self, field: &str) -> Option<String>;

    fn setup_secret(&self) -> Option<String> {
        self.setup(ENVIRONMENT_SETUP_SECRET)
    }

}

impl SetupEnvironment for Environment {

    fn setup(&self, field: &str) -> Option<String> {
        self.variable(ENVIRONMENT_SETUP, field)
    }

}