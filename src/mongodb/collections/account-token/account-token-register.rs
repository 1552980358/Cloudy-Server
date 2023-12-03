use mongodb::Collection;
use crate::mongodb::collections::{Account, AccountToken};

#[async_trait]
pub trait Register {
    async fn register(&self, account: Account, issue: usize, duration: usize) -> Option<AccountToken>;
}

#[async_trait]
impl Register for Collection<AccountToken> {
    async fn register(&self, account: Account, issue: usize, duration: usize) -> Option<AccountToken> {
        let account_token = AccountToken::new(account, issue, duration);
        self.insert_one(&account_token, None)
            .await
            .map(|_| { account_token })
            .ok()
    }
}