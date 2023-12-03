#[path = "auth/auth-post.rs"]
mod auth_post;
use auth_post::post;

use rocket::Route;

const ROUTE_AUTH: &str = "/auth";
pub fn route<'a>() -> &'a str {
    ROUTE_AUTH
}

pub fn routes() -> Vec<Route> {
    routes![
        post,
    ]
}