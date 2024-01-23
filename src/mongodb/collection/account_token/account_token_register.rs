use mongodb::Collection;
use crate::mongodb::collection::{Account, AccountToken};

type Result = mongodb::error::Result<AccountToken>;

#[async_trait]
pub trait Register {
    async fn register(
        &self,
        account: Account,
        issue: u64,
        duration: u64,
        renewal: bool
    ) -> Result;
}

#[async_trait]
impl Register for Collection<AccountToken> {
    async fn register(
        &self,
        account: Account,
        issue: u64,
        duration: u64,
        renewal: bool
    ) -> Result {
        let account_token = AccountToken::new(
            account, issue, duration, renewal
        );
        self.insert_one(&account_token, None)
            .await
            .map(|_| account_token)
    }
}