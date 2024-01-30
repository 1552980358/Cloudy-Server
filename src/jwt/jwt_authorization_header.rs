use regex::{Error, Regex};
use Rocket::Request;
use crate::rocket::RequestHeader;

type Result = std::result::Result<Option<String>, Error>;

pub trait JWTAuthorizationHeader {
    fn jwt_authorization_header(&self) -> Result;
}

const AUTHORIZATION_REGEX: &str = r"JWT (?<jwt>([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_=]+)\.([a-zA-Z0-9_\-+/=]*))";
const AUTHORIZATION_REGEX_JWT: &str = "jwt";
impl JWTAuthorizationHeader for Request<'_> {
    fn jwt_authorization_header(&self) -> Result {
        let Some(authorization) = self.authorization() else {
            return Ok(None)
        };
        let regex = Regex::new(AUTHORIZATION_REGEX)?;
        let jwt = regex.captures(&authorization)
            .map(|captures| captures.name(AUTHORIZATION_REGEX_JWT))
            .flatten()
            .map(|jwt_match| jwt_match.as_str().to_string());
        Ok(jwt)
    }
}