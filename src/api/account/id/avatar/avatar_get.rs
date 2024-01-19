use base64::Engine;
use base64::engine::general_purpose::STANDARD as StandardBase64;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneOptions;
use Rocket::http::{ContentType, Status};
use Rocket::State;
use serde::Deserialize;

use crate::mongodb::collection::account::{
    Account,
    Field as AccountField
};
use crate::mongodb::MongoDB;

#[derive(Deserialize, Debug)]
struct AvatarView {
    pub avatar: Option<String>
}

#[get("/<account_id>/avatar", rank = 2)]
pub async fn get_account_avatar(
    mongodb: &State<MongoDB>, account_id: &str
) -> Result<(ContentType, Vec<u8>), Status> {
    let object_id = ObjectId::parse_str(account_id)
        .map_err(|_| Status::InternalServerError)?;
    let filter = doc! {
        AccountField::id(): object_id,
    };
    let projection = doc! {
        AccountField::id(): 0,
        AccountField::avatar(): 1,
    };
    let find_one_option = FindOneOptions::builder()
        .projection(projection)
        .build();

    let avatar = mongodb.view::<Account, AvatarView>()
        .find_one(filter, find_one_option)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(|avatar_view| avatar_view.avatar)
        .flatten()
        .ok_or_else(|| Status::NotFound)?;

    let avatar = handle_avatar_type(avatar).ok_or_else(|| Status::NotFound)?;
    let avatar_base64 = avatar.1;
    let avatar_type = avatar.0;
    StandardBase64.decode(avatar_base64)
        .map(|bytes| (avatar_type, bytes))
        .map_err(|_| Status::InternalServerError)
}

const PREFIX_JPEG: &str = "data:image/jpeg;base64,";
const PREFIX_PNG: &str = "data:image/png;base64,";
fn handle_avatar_type(avatar: String) -> Option<(ContentType, String)> {
    if avatar.starts_with(PREFIX_JPEG) {
        let avatar_base64 = avatar_split_prefix(&avatar, PREFIX_JPEG);
        return Some((ContentType::JPEG, avatar_base64))
    }
    if avatar.starts_with(PREFIX_PNG) {
        let avatar_base64 = avatar_split_prefix(&avatar, PREFIX_PNG);
        return Some((ContentType::PNG, avatar_base64))
    }
    None
}

fn avatar_split_prefix(avatar: &String, prefix: &str) -> String {
    avatar.as_str()[prefix.len()..].to_string()
}