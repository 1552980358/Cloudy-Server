#[macro_use]
extern crate rocket;

mod mongodb;
use mongodb::Mongodb;

mod jwt;
use jwt::JWT;

mod util;

mod api;
use api::{setup, auth};

#[launch]
async fn server() -> _ {
    let mongodb = Mongodb::connect().await;
    let jwt = JWT::setup().await;

    rocket::build()
        .manage(mongodb)
        .manage(jwt)
        .mount(setup::route(), setup::routes())
        .mount(auth::route(), auth::routes())
}