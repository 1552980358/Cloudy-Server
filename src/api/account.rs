use Rocket::Route;

#[path = "account/account-get.rs"]
mod account_get;
use account_get::get;

mod id;

mod find;

const ROUTE_ACCOUNT: &str = "/account";
pub fn route<'a>() -> &'a str {
    ROUTE_ACCOUNT
}

pub fn routes() -> Vec<Route> {
    routes![
        /* /account */
        get,

        /* /account/<account_id> */
        id::get,

        /* /account/<account_id>/avatar */
        id::avatar::get,

        /* /account/find/<account_identifier> */
        find::get,
    ]
}