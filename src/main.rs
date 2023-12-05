#[macro_use]
extern crate rocket as Rocket;

mod mongodb;
use mongodb::Mongodb;

mod jwt;
use jwt::JWT;

mod util;

mod api;
use api::{setup, auth};

#[cfg(not(debug_assertions))]
#[launch]
async fn server() -> _ {
    let mongodb = Mongodb::connect().await;
    let jwt = JWT::setup().await;

    Rocket::build()
        .manage(mongodb)
        .manage(jwt)
        .mount(setup::route(), setup::routes())
        .mount(auth::route(), auth::routes())
}

#[cfg(debug_assertions)]
mod rocket;
#[cfg(debug_assertions)]
use rocket::CORS;

#[cfg(debug_assertions)]
#[launch]
async fn server() -> _ {
    let mongodb = Mongodb::connect().await;
    let jwt = JWT::setup().await;

    Rocket::build()
        // CORS only for debug use
        .attach(CORS)
        .manage(mongodb)
        .manage(jwt)
        .mount(setup::route(), setup::routes())
        .mount(auth::route(), auth::routes())
}