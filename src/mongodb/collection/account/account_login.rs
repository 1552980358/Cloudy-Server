use mongodb::bson::to_document;
use mongodb::Collection;
use serde::Serialize;

use crate::mongodb::collection::Account;

type Result = mongodb::error::Result<Option<Account>>;

#[derive(Serialize, Debug)]
struct Filter {
    username: String,
    password: String,
}

#[async_trait]
pub trait Login {
    async fn login(&self, username: String, password: String) -> Result;
}

#[async_trait]
impl Login for Collection<Account> {
    async fn login(&self, username: String, password: String) -> Result {
        let filter = Filter { username, password };
        let filter_document = to_document(&filter)?;

        self.find_one(filter_document, None).await
    }
}