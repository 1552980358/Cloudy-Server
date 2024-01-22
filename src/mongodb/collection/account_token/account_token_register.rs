use mongodb::Collection;
use crate::mongodb::collection::{Account, AccountToken};

type Result = mongodb::error::Result<AccountToken>;

#[async_trait]
pub trait Register {
    async fn register(&self, account: Account, issue: u64, duration: u64) -> Result;
}

#[async_trait]
impl Register for Collection<AccountToken> {
    async fn register(&self, account: Account, issue: u64, duration: u64) -> Result {
        let account_token = AccountToken::new(account, issue, duration);
        self.insert_one(&account_token, None)
            .await
            .map(|_| account_token)
    }
}