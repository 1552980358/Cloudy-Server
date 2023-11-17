use std::fmt::Display;
use mongodb::Collection;
use crate::mongodb::collections::Account;

mod mongodb_env;
mod mongodb_connect;

mod collections;

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