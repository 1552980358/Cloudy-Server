use std::fmt::Display;
use mongodb::Collection;

#[path = "mongodb/mongodb-env.rs"]
mod env;
#[path = "mongodb/mongodb-connect.rs"]
mod connect;

pub mod collections;

#[path = "mongodb/mongodb-object-id.rs"]
mod object_id;

use collections::{Account, AccountToken};

const PANIC: &str = "Mongodb Panic: ";
fn self_panic<M: Display>(message: M) -> ! {
    panic!("{}{}", PANIC, message)
}

/**
 * [Mongodb]
 **/
pub struct Mongodb {

    pub account: Collection<Account>,

    pub account_token: Collection<AccountToken>,

}

impl Mongodb {

    fn new(
        account: Collection<Account>,
        account_token: Collection<AccountToken>,
    ) -> Self {
        Self {
            account,
            account_token
        }
    }

}