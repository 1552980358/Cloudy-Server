#[macro_use]
extern crate rocket as Rocket;

mod mongodb;
use mongodb::MongoDB;

mod jwt;
use jwt::JWT;

mod util;

mod api;
use api::RocketMountApi;

mod rocket;

mod environment;
use environment::Environment;

#[cfg(not(debug_assertions))]
#[launch]
async fn server() -> _ {
    let environment = Environment::setup();
    let mongodb = MongoDB::build();
    let jwt = JWT::setup();

    Rocket::build()
        .manage(environment.await)
        .manage(mongodb.await)
        .manage(jwt.await)
        .mount_api()
}

#[cfg(debug_assertions)]
use rocket::{CORS, OPTIONS};

#[cfg(debug_assertions)]
#[launch]
async fn server() -> _ {
    let environment = Environment::setup();
    let mongodb = MongoDB::build();
    let jwt = JWT::setup();

    Rocket::build()
        // CORS only for debug use
        .attach(CORS)
        // OPTIONS method only for debug use
        .attach(OPTIONS)
        .manage(environment.await)
        .manage(mongodb.await)
        .manage(jwt.await)
        .mount_api()
}