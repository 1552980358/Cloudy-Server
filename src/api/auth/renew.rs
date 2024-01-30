use Rocket::Route;

mod renew_get;
use renew_get::renew_token as get;
use crate::api::ChildRouteVec;

const ROUTE_RENEW: &str = "/renew";
pub fn routes() -> Vec<Route> {
    routes![get].add_parent_route(ROUTE_RENEW)
}