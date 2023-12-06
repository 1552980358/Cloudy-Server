use Rocket::fairing::{Fairing, Info, Kind};
use Rocket::{Request, Response};
use Rocket::http::{Method, Status};

pub struct OPTIONS;

#[async_trait]
impl Fairing for OPTIONS {

    fn info(&self) -> Info {
        Info {
            name: "Debug OPTIONS method attached",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // OPTIONS method only for debug use
        if request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }

}