use mongodb::{Collection, Database};

#[path = "mongodb/mongodb-environment.rs"]
mod environment;

#[path = "mongodb/mongodb-setup.rs"]
mod setup;

pub mod collection;
use collection::Collection as MongoDBCollection;

#[path = "mongodb/mongodb-object-id.rs"]
mod object_id;

#[path = "mongodb/mongodb-ping.rs"]
mod ping;

#[path = "mongodb/mongodb-state.rs"]
mod state;
pub use state::MongoDBState;

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

    pub fn view<C: MongoDBCollection, D>(&self) -> Collection<D> {
        self.database.collection::<D>(C::name())
    }

}