use rocket::{Build, Rocket};

mod auth;

mod setup;

pub trait RocketMountApi {
    fn mount_api(self) -> Self;
}

impl RocketMountApi for Rocket<Build> {
    fn mount_api(self) -> Self {
        self.mount(auth::route(), auth::routes())
            .mount(setup::route(), setup::routes())
    }
}