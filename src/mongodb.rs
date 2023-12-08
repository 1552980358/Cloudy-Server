use mongodb::{Collection, Database};

#[path = "mongodb/mongodb-env.rs"]
mod env;
#[path = "mongodb/mongodb-build.rs"]
mod build;

pub mod collection;
use collection::Collection as MongoDBCollection;

#[path = "mongodb/mongodb-object-id.rs"]
mod object_id;

#[path = "mongodb/mongodb-ping.rs"]
mod ping;

macro_rules! mongodb_panic {
    ($display:expr) => {
        panic!("Mongodb Panic: {}", $display)
    };
    ($($arg:tt)*) => {
        panic!("Mongodb Panic: {}", format!($($arg)*))
    };
}
use mongodb_panic;

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