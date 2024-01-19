use mongodb::bson::doc;
use mongodb::Collection;
use crate::mongodb::collection::Account;

type LoginResult = mongodb::error::Result<Option<Account>>;

#[async_trait]
pub trait Login {
    async fn login(&self, username: String, password: String) -> LoginResult;
}

const FILTER_USERNAME: &str = "username";
const FILTER_PASSWORD: &str = "password";
#[async_trait]
impl Login for Collection<Account> {
    async fn login(&self, username: String, password: String) -> LoginResult {
        let filter = doc! {
            FILTER_USERNAME: username,
            FILTER_PASSWORD: password,
        };
        self.find_one(filter, None).await
    }
}