use mongodb::bson::{doc, to_bson};
use mongodb::Collection;

use crate::mongodb::collection::{Account, account::Role};

#[async_trait]
pub trait FindOwner {
    async fn find_owner(&self) -> bool;
}

const FILTER_ROLE: &str = "role";
#[async_trait]
impl FindOwner for Collection<Account> {
    async fn find_owner(&self) -> bool {
        let Ok(role) = to_bson(&Role::Owner) else {
            return false;
        };
        let filter = doc! {
            FILTER_ROLE: role
        };

        self.find_one(filter, None).await
            .ok()
            .map(|account| account.is_some())
            .unwrap_or_else(|| false)
    }
}