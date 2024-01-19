use Rocket::response::Redirect;

use crate::jwt::Auth;

#[get("/")]
pub async fn get_authed_account_metadata(auth: Auth) -> Redirect {
    Redirect::to(format!("/account/{}", auth.0))
}