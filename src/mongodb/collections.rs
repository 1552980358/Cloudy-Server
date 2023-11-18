mod account;
pub use account::{Account};

#[path = "collections/account-token.rs"]
mod account_token;
pub use account_token::AccountToken;

trait Collection {
    fn id(&self) -> String;
}

impl Collection for Account {
    fn id(&self) -> String {
        self._id.to_hex()
    }
}

impl Collection for AccountToken {
    fn id(&self) -> String {
        self._id.to_hex()
    }
}