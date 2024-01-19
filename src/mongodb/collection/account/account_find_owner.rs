use mongodb::bson::{doc, to_bson};
use mongodb::Collection;
use mongodb::error::Result;

use crate::mongodb::collection::Account;
use crate::mongodb::collection::account::{
    Field as AccountField,
    Role
};

#[async_trait]
pub trait FindOwner {

    async fn find_owner(&self) -> Result<Option<Account>>;

    async fn has_owner(&self) -> bool;

}

#[async_trait]
impl FindOwner for Collection<Account> {

    async fn find_owner(&self) -> Result<Option<Account>> {
        let role = to_bson(&Role::Owner).map_err(|_| {
            use mongodb::error::Error;
            use mongodb::error::ErrorKind::BsonSerialization;
            Error::custom(BsonSerialization)
        })?;

        let filter = doc! {
            AccountField::role(): role,
        };

        self.find_one(filter, None)
            .await
    }

    async fn has_owner(&self) -> bool {
        self.find_owner()
            .await
            .map(|account| account.is_some())
            .unwrap_or_else(|_| false)
    }

}