use Rocket::Route;

const ROUTE_ACCOUNT: &str = "/account";
pub fn route<'a>() -> &'a str {
    ROUTE_ACCOUNT
}

pub fn routes() -> Vec<Route> {
    routes![
    ]
}