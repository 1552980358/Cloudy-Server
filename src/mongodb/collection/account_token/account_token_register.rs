use mongodb::Collection;
use crate::mongodb::collection::AccountToken;

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
}