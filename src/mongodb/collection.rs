pub mod account;
pub use account::Account;

pub mod account_token;
pub use account_token::AccountToken;

pub trait Collection {
    fn name<'a>() -> &'a str;
}