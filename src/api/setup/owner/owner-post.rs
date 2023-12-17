use Rocket::data::{FromData, Outcome};
use Rocket::http::{Status};
use Rocket::{Data, Request, State};

use crate::jwt::JWT;
use crate::mongodb::collection::Account;
use crate::mongodb::collection::account::{AccountCollection, FindOwner};
use crate::mongodb::MongoDB;
use crate::rocket::{PostData, RequestHeader};

pub struct OwnerRequestBody(Account);

#[post("/owner?<secret>", data = "<owner_request_body>")]
pub async fn post(
    jwt: &State<JWT>, mongodb: &State<MongoDB>, secret: String, owner_request_body: OwnerRequestBody
) -> Result<Status, Status> {
    if jwt.secret != secret {
        return Err(Status::Unauthorized);
    }

    let account_collection = mongodb.account();

    // If owner set, a new owner is not allowed to configure
    if account_collection.has_owner().await {
        return Err(Status::Forbidden);
    }

    let account = owner_request_body.0;
    let insert = account_collection.insert_one(&account, None).await;
    let is_object_id_valid = insert.map(|insert| insert.inserted_id)
        .map(|inserted_id| inserted_id.as_object_id())
        .ok()
        .flatten()
        .map(|object_id| object_id.to_hex() == account.id);

    match is_object_id_valid {
        Some(true) => { Ok(Status::Ok) }
        _ => { Err(Status::InternalServerError) }
    }
}

type OwnerRequestBodyDecodeResult = serde_json::error::Result<OwnerRequestBody>;
impl OwnerRequestBody {
    fn from(request_data: String) -> OwnerRequestBodyDecodeResult {
        serde_json::from_str::<Account>(&*request_data)
            .map(|account| OwnerRequestBody(account))
    }
}

#[derive(Debug)]
pub enum OwnerRequestError {
    InvalidContentType,
    ContentLengthMissing,
    ContentReadError,
    JSONDecodeError
}

#[async_trait]
impl<'a> FromData<'a> for OwnerRequestBody {
    type Error = OwnerRequestError;

    async fn from_data(request: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self> {
        if !request.is_json_content() {
            return Outcome::Error(
                (Status::BadRequest, OwnerRequestError::InvalidContentType)
            )
        }
        let Some(content_length) = request.content_length() else {
            return Outcome::Error(
                (Status::BadRequest, OwnerRequestError::ContentLengthMissing)
            )
        };

        let Ok(request_data) = data.string(content_length).await else {
            return Outcome::Error(
                (Status::BadRequest, OwnerRequestError::ContentReadError)
            )
        };

        OwnerRequestBody::from(request_data)
            .map(|owner_request_body| {
                Outcome::Success(owner_request_body)
            })
            .unwrap_or_else(|_| {
                Outcome::Error(
                    (Status::BadRequest, OwnerRequestError::JSONDecodeError)
                )
            })
    }
}