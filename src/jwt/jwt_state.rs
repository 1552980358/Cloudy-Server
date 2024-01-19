use Rocket::Request;

use crate::jwt::JWT;

pub trait JWTState {

    fn jwt(&self) -> Option<&JWT>;

}

impl JWTState for Request<'_> {

    fn jwt(&self) -> Option<&JWT> {
        self.rocket().state()
    }

}