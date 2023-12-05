use Rocket::http::ContentType;
use Rocket::Request;

pub trait PostRequestHeader {

    fn is_json_content(&self) -> bool;

    fn content_length(&self) -> Option<i32>;

}

const HEADER_CONTENT_LENGTH: &str = "Content-Length";

impl PostRequestHeader for Request<'_> {

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