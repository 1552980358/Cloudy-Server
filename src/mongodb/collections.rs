mod account;
pub use account::{Account};

pub trait Collection {
    fn id(&self) -> String;
}

impl Collection for Account {
    fn id(&self) -> String {
        self._id.to_hex()
    }
}