use std::fmt::Display;
use mongodb::{Collection, Database};

#[path = "mongodb/mongodb-env.rs"]
mod env;
#[path = "mongodb/mongodb-build.rs"]
mod build;

pub mod collection;
use collection::Collection as MongoDBCollection;

#[path = "mongodb/mongodb-object-id.rs"]
mod object_id;

const PANIC: &str = "Mongodb Panic: ";
fn self_panic<M: Display>(message: M) -> ! {
    panic!("{}{}", PANIC, message)
}

/**
 * [MongoDB]
 **/
pub struct MongoDB {

    database: Database,

}

impl MongoDB {

    fn new(
        database: Database
    ) -> Self {
        Self { database }
    }

    pub fn collection<C: MongoDBCollection>(&self) -> Collection<C> {
        self.database.collection::<C>(C::name())
    }

}