use std::fmt::Display;

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
}

impl Mongodb {

    fn new() -> Self {
        Self {}
    }

}