use rocket::Route;

use crate::concat_vec;

mod auth_post;
use auth_post::login_auth as post;

mod renew;

const ROUTE_AUTH: &str = "/auth";
pub fn route<'a>() -> &'a str {
    ROUTE_AUTH
}

pub fn routes() -> Vec<Route> {
    concat_vec![
        routes![post],
        renew::routes()
    ]
}