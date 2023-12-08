pub mod account;
pub use account::Account;

#[path = "collection/account-token.rs"]
pub mod account_token;
pub use account_token::AccountToken;

pub trait Collection {
    fn name<'a>() -> &'a str;
}