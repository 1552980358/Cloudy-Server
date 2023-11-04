#[macro_use]
extern crate rocket;

mod mongodb;
use mongodb::Mongodb;

mod util;

#[launch]
async fn server() -> _ {
    let mongodb = Mongodb::connect().await;

    rocket::build()
        .manage(mongodb)
}