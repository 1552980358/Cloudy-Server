use Rocket::http::ContentType;
use Rocket::Request;

pub trait Header {

    fn authorization(&self) -> Option<String>;

    fn is_json_content(&self) -> bool;

    fn content_length(&self) -> Option<i32>;

}

const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_LENGTH: &str = "Content-Length";

impl Header for Request<'_> {

    fn authorization(&self) -> Option<String> {
        self.headers().get_one(HEADER_AUTHORIZATION)
            .map(|authorization| authorization.to_string())
    }

    fn is_json_content(&self) -> bool {
        self.content_type()
            .map(|content_type| *content_type == ContentType::JSON)
            .unwrap_or_else(|| false)
    }

    fn content_length(&self) -> Option<i32> {
        self.headers().get_one(HEADER_CONTENT_LENGTH)
            .map(|content_length| content_length.parse::<i32>().ok())
            .flatten()
    }

}