mod find_get;

use Rocket::Route;
use find_get::find_account as get;
use crate::api::ChildRouteVec;

const ROUTE_FIND: &str = "/find";
pub fn routes() -> Vec<Route> {
    routes![get].add_parent_route(ROUTE_FIND)
}