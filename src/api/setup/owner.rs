use crate::api::ChildRouteVec;

mod owner_get;

use rocket::Route;
use owner_get::check_owner_state as get;

mod owner_post;
use owner_post::setup_owner as post;

const ROUTE_OWNER: &str = "/owner";
pub fn routes() -> Vec<Route> {
    routes![get, post]
        .add_parent_route(ROUTE_OWNER)
}