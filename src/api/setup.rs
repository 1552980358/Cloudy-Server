#[path = "setup/setup-get.rs"]
mod setup_get;
use setup_get::get;

use rocket::Route;

const ROUTE_SETUP: &str = "/setup";
pub fn route<'a>() -> &'a str {
    ROUTE_SETUP
}

pub fn routes() -> Vec<Route> {
    routes![
        get,
    ]
}