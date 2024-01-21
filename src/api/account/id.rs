mod id_get;

use Rocket::Route;
use id_get::get_account_metadata as get;

mod avatar;

pub fn routes() -> Vec<Route> {
    routes![
        get,
        avatar::get
    ]
}