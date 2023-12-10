use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use mongodb::Collection;

use crate::mongodb::collection::account_token::{
    AccountToken,
    field as AccountTokenField
};

type FindAccountResult = Result<Option<String>>;

#[async_trait]
pub trait FindAccount {

    async fn find_account(&self, tok: String, iat: usize, exp: usize) -> FindAccountResult;

}


#[async_trait]
impl FindAccount for Collection<AccountToken> {
    async fn find_account(&self, tok: String, iat: usize, exp: usize) -> FindAccountResult {
        let object_id = ObjectId::parse_str(tok)
            .map_err(|_| {
                use mongodb::error::{Error, ErrorKind};
                Error::custom(ErrorKind::BsonSerialization)
            })?;

        let filter = doc! {
            AccountTokenField::id(): object_id,
            AccountTokenField::issue(): iat as i64,
            AccountTokenField::expiry(): exp as i64,
        };

        let account = self.find_one(filter, None).await?
            .map(|account_token| account_token.account);
        Ok(account)
    }
}