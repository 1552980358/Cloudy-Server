use mongodb::bson::to_document;
use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use mongodb::Collection;
use serde::Serialize;

use crate::mongodb::collection::account_token::AccountToken;

type Result = mongodb::error::Result<Option<String>>;

#[derive(Serialize, Debug)]
struct Filter {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    id: String,
    issue: u64,
    expiry: u64,
}

#[async_trait]
pub trait FindAccount {
    async fn find_account(&self, tok: String, iat: u64, exp: u64) -> Result;
}

#[async_trait]
impl FindAccount for Collection<AccountToken> {
    async fn find_account(&self, tok: String, iat: u64, exp: u64) -> Result {
        let filter = Filter {
            id: tok, issue: iat, expiry: exp
        };
        let filter_document = to_document(&filter)?;

        self.find_one(filter_document, None)
            .await
            .map(|account_token_option| {
                account_token_option.map(|account_token| {
                    account_token.account
                })
            })
    }
}