use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::{Data, Request};
use rocket::http::{Status};

use crate::api::auth::auth_post::AuthRequestBody;
use crate::rocket::RequestHeader;

type AuthRequestBodyDecodeResult = serde_json::error::Result<AuthRequestBody>;
impl AuthRequestBody {
    fn decode(request_body: String) -> AuthRequestBodyDecodeResult {
        serde_json::from_str(&*request_body)
    }
}

#[derive(Debug)]
pub enum AuthRequestError {
    InvalidContentType,
    ContentLengthMissing,
    InvalidContent,
    InvalidJSONContent
}

#[async_trait]
impl<'r> FromData<'r> for AuthRequestBody {

    type Error = AuthRequestError;

    async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        if !request.is_json_content() {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::InvalidContentType)
            )
        }

        let Some(content_length) = request.content_length() else {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::ContentLengthMissing)
            )
        };

        let Ok(request_body) = data.read_request_body(content_length).await else {
            return Outcome::Error(
                (Status::BadRequest, AuthRequestError::InvalidContent)
            )
        };

        AuthRequestBody::decode(request_body)
            .map(|auth_request_body| {
                Outcome::Success(auth_request_body)
            })
            .unwrap_or_else(|_| {
                Outcome::Error(
                    (Status::BadRequest, AuthRequestError::InvalidJSONContent)
                )
            })
    }

}

type AuthRequestReadDataError = std::io::Result<String>;
#[async_trait]
trait AuthRequestData {
    async fn read_request_body(self, content_length: i32) -> AuthRequestReadDataError;
}

#[async_trait]
impl<'r> AuthRequestData for Data<'r> {
    async fn read_request_body(self, content_length: i32) -> AuthRequestReadDataError {
        self.open(content_length.bytes())
            .into_string()
            .await
            .map(|request_data| request_data.value)
    }
}