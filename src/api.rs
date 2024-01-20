use rocket::{Build, Rocket};

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