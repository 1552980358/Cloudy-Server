#[path = "auth/auth-post.rs"]
mod auth_post;

use rocket::Route;
use auth_post::post;

const ROUTE_API: &str = "/auth";

pub fn route<'a>() -> &'a str {
    ROUTE_API
}

pub fn routes() -> Vec<Route> {
    return routes![
        post,
    ]
}