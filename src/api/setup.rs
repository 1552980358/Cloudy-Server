#[path = "setup/setup-get.rs"]
mod setup_get;
use setup_get::get;

mod owner;

use rocket::Route;

use crate::environment::Environment;

const ROUTE_SETUP: &str = "/setup";
pub fn route<'a>() -> &'a str {
    ROUTE_SETUP
}

pub fn routes() -> Vec<Route> {
    routes![
        /* /setup */
        get,

        /* /setup/owner */
        owner::get,
        owner::post,
    ]
}

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