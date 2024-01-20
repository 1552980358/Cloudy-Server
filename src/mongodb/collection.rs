pub mod account;
pub use account::Account;

pub mod account_token;
pub use account_token::AccountToken;

mod account_directory;
pub use account_directory::AccountDirectory;

mod account_file;
pub use account_file::AccountFile;

pub trait Collection {
    fn name<'a>() -> &'a str;
}