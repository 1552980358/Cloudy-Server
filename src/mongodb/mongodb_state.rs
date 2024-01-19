use Rocket::Request;

use crate::mongodb::MongoDB;

pub trait MongoDBState {

    fn mongodb(&self) -> Option<&MongoDB>;

}

impl MongoDBState for Request<'_> {

    fn mongodb(&self) -> Option<&MongoDB> {
        self.rocket().state()
    }

}