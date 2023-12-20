#[path = "auth/auth-post.rs"]
mod auth_post;
use auth_post::post;

#[path = "auth/auth-get.rs"]
mod auth_get;
use auth_get::get;

use rocket::Route;

const ROUTE_AUTH: &str = "/auth";
pub fn route<'a>() -> &'a str {
    ROUTE_AUTH
}

pub fn routes() -> Vec<Route> {
    routes![
        get, post,
    ]
}