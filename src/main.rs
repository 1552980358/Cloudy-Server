#[macro_use]
extern crate rocket as Rocket;

mod mongodb;
use mongodb::MongoDB;

mod jwt;
use jwt::JWT;

mod api;
use api::RocketMountApi;

mod rocket;

mod environment;
mod util;

use environment::Environment;

#[cfg(not(debug_assertions))]
#[launch]
async fn server() -> _ {
    let environment = Environment::setup();
    let mongodb = MongoDB::setup(&environment);
    let jwt = JWT::setup(&environment);

    // Have a ping test first
    let _ = mongodb.ping().await;

    Rocket::build()
        .manage(environment)
        .manage(mongodb)
        .manage(jwt)
        .mount_api()
}

#[cfg(debug_assertions)]
use rocket::{CORS, OPTIONS};

#[cfg(debug_assertions)]
#[launch]
async fn server() -> _ {
    let environment = Environment::setup();
    let mongodb = MongoDB::setup(&environment);
    let jwt = JWT::setup(&environment);

    // Have a ping test first
    let _ = mongodb.ping().await;

    Rocket::build()
        // CORS only for debug use
        .attach(CORS)
        // OPTIONS method only for debug use
        .attach(OPTIONS)
        .manage(environment)
        .manage(mongodb)
        .manage(jwt)
        .mount_api()
}