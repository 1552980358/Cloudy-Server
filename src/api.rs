use rocket::{Build, Rocket};

mod auth;

mod setup;

mod account;

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