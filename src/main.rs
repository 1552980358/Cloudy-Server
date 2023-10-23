#[macro_use]
extern crate rocket;

#[launch]
async fn server() -> _ {
    rocket::build()
}