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
    vec![
        /* /account */
        routes![get],
        /* /account/<account_id> */
        id::routes(),
        /* /account/find */
        find::routes()
    ].concat()
}