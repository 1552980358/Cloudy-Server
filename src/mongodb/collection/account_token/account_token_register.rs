use mongodb::Collection;
use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use mongodb::bson::{doc, to_document};
use serde::Serialize;

use crate::mongodb::collection::account_token::{
    AccountToken,
    Field as AccountTokenField
};
use crate::mongodb::Operator;

type Result<T> = mongodb::error::Result<T>;

#[async_trait]
pub trait Register {

    async fn register_new(
        &self,
        account_id: &String,
        issue: &u64,
        duration: &u64,
        renewal: &bool
    ) -> Result<AccountToken>;

    async fn register_renewal(
        &self,
        account_id: &String,
        token_id: &String,
        issue: &u64,
        expiry: &u64,
    ) -> Result<Option<AccountToken>>;

}

#[derive(Serialize, Debug)]
struct Filter {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    id: String,
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    account: String,
    issue: u64,
    renewal: bool,
}

#[async_trait]
impl Register for Collection<AccountToken> {

    async fn register_new(
        &self,
        account_id: &String,
        issue: &u64,
        duration: &u64,
        renewal: &bool
    ) -> Result<AccountToken> {
        let account_token = AccountToken::new(
            account_id.to_owned(), *issue, *duration, *renewal
        );
        self.insert_one(&account_token, None)
            .await
            .map(|_| account_token)
    }

    async fn register_renewal(
        &self,
        account_id: &String,
        token_id: &String,
        issue: &u64,
        expiry: &u64,
    ) -> Result<Option<AccountToken>> {
        let filter = Filter {
            id: token_id.to_owned(),
            account: account_id.to_owned(),
            issue: *issue,
            renewal: true
        };

        let filter_document = to_document(&filter)?;
        let update_document = doc! {
            Operator::field_update_set(): doc! {
                AccountTokenField::expiry(): *expiry as i64,
            }
        };

        // Update expiry field
        let _ = self.update_one(filter_document, update_document, None)
            .await?;

        let filter_document = to_document(&filter)?;
        // Do a validation filter as success checking
        self.find_one(filter_document, None).await
    }

}