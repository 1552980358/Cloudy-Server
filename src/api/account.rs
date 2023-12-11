use Rocket::Route;

#[path = "account/account-get.rs"]
mod account_get;
use account_get::get;

const ROUTE_ACCOUNT: &str = "/account";
pub fn route<'a>() -> &'a str {
    ROUTE_ACCOUNT
}

pub fn routes() -> Vec<Route> {
    routes![
        get,
    ]
}