use mongodb::{Collection, Database};

mod mongodb_environment;
use mongodb_environment::MongoDBEnvironment;

mod mongodb_setup;

pub mod collection;
use collection::Collection as MongoDBCollection;

mod mongodb_object_id;
use mongodb_object_id as object_id;

mod mongodb_ping;

mod mongodb_state;
pub use mongodb_state::MongoDBState;

mod mongodb_filter;
pub use mongodb_filter::Filter;

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