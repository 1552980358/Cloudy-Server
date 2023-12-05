use Rocket::Data;
use Rocket::data::ToByteUnit;
use std::io::{Error as IOError, ErrorKind::InvalidInput, Result as IOResult};

#[async_trait]
pub trait PostData {
    async fn string(self, content_length: i32) -> PostDataReadStringResult;
}

type PostDataReadStringResult = IOResult<String>;

#[async_trait]
impl<'r> PostData for Data<'r> {
    async fn string(self, content_length: i32) -> PostDataReadStringResult {
        if content_length < 0 {
            return Err(IOError::from(InvalidInput))
        }
        self.open(content_length.bytes())
            .into_string()
            .await
            .map(|capped| capped.value)
    }
}