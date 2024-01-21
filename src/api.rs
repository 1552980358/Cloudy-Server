use rocket::{Build, Rocket};
use rocket::Route;

mod account;

mod auth;

mod setup;

macro_rules! api_panic {
    ($display:expr) => {
        panic!("API Panic: {}", $display)
    };
    ($($arg:tt)*) => {
        panic!("API Panic: {}", format!($($arg)*))
    };
}
use api_panic;

pub trait RocketMountApi {
    fn mount_api(self) -> Self;
}

impl RocketMountApi for Rocket<Build> {
    fn mount_api(self) -> Self {
        self.mount(auth::route(), auth::routes())
            .mount(setup::route(), setup::routes())
            .mount(account::route(), account::routes())
    }
}

trait ChildRouteVec {
    fn add_parent_route(&self, route: &str) -> Vec<Route>;
}

impl ChildRouteVec for Vec<Route> {
    fn add_parent_route(&self, parent: &str) -> Vec<Route> {
        self.iter()
            .filter_map(|route| {
                route.clone()
                    .map_base(|path| format!("{}{}", parent, path))
                    .ok()
            })
            .collect()
    }
}