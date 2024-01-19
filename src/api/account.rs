use Rocket::Route;

mod account_get;
use account_get::get_authed_account_metadata as get;

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