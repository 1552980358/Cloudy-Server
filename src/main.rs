#[macro_use]
extern crate rocket as Rocket;

mod mongodb;
use mongodb::MongoDB;

mod jwt;
use jwt::JWT;

mod util;

mod api;
use api::{setup, auth};

#[cfg(not(debug_assertions))]
#[launch]
async fn server() -> _ {
    let mongodb = MongoDB::build();
    let jwt = JWT::setup();

    Rocket::build()
        .manage(mongodb)
        .manage(jwt.await)
        .mount(setup::route(), setup::routes())
        .mount(auth::route(), auth::routes())
}

#[cfg(debug_assertions)]
mod rocket;
#[cfg(debug_assertions)]
use rocket::{CORS, OPTIONS};

#[cfg(debug_assertions)]
#[launch]
async fn server() -> _ {
    let mongodb = MongoDB::build();
    let jwt = JWT::setup();

    Rocket::build()
        // CORS only for debug use
        .attach(CORS)
        // OPTIONS method only for debug use
        .attach(OPTIONS)
        .manage(mongodb)
        .manage(jwt.await)
        .mount(setup::route(), setup::routes())
        .mount(auth::route(), auth::routes())
}