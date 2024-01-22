use mongodb::bson::to_document;
use mongodb::Collection;
use serde::Serialize;

use crate::mongodb::collection::account::{
    Account,
    Role
};

#[derive(Serialize, Debug)]
struct Filter {
    role: Role
}

type Result = mongodb::error::Result<Option<Account>>;

#[async_trait]
pub trait FindOwner {

    async fn find_owner(&self) -> Result;

    async fn has_owner(&self) -> bool;

}

#[async_trait]
impl FindOwner for Collection<Account> {

    async fn find_owner(&self) -> Result {
        let filter = Filter {
            role: Role::Owner
        };
        let filter_document = to_document(&filter)?;

        self.find_one(filter_document, None)
            .await
    }

    async fn has_owner(&self) -> bool {
        self.find_owner()
            .await
            .map(|account| account.is_some())
            .unwrap_or_else(|_| false)
    }

}