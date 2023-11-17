use std::fmt::Display;
use mongodb::Collection;

#[path = "mongodb/mongodb-env.rs"]
mod env;
#[path = "mongodb/mongodb-connect.rs"]
mod connect;

mod collections;
use collections::Account;

const PANIC: &str = "Mongodb Panic: ";
fn self_panic<M: Display>(message: M) -> ! {
    panic!("{}{}", PANIC, message)
}

/**
 * [Mongodb]
 **/
pub struct Mongodb {

    pub account: Collection<Account>,

}

impl Mongodb {

    fn new(
        account: Collection<Account>,
    ) -> Self {
        Self {
            account
        }
    }

}