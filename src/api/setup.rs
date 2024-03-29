use rocket::Route;

mod setup_get;
use setup_get::check_setup_state as get;

mod owner;

mod setup_environment;
use setup_environment::SetupEnvironment;
use crate::concat_vec;

const ROUTE_SETUP: &str = "/setup";
pub fn route<'a>() -> &'a str {
    ROUTE_SETUP
}

pub fn routes() -> Vec<Route> {
    concat_vec![
        /* /setup */
        routes![get],
        /* /setup/owner */
        owner::routes()
    ]
}