use mongodb::bson::doc;
use Rocket::response::Redirect;

use crate::jwt::Auth;

#[get("/")]
pub async fn get(auth: Auth) -> Redirect {
    Redirect::to(format!("/account/{}", auth.0))
}