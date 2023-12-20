use Rocket::http::Status;

use crate::jwt::Auth;

#[get("/")]
pub async fn get(_auth: Auth) -> Status {
    Status::Ok
}