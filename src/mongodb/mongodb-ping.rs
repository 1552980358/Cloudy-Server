use mongodb::bson::doc;
use crate::mongodb::{MongoDB, mongodb_panic};

const COMMAND_PING: &str = "ping";
impl MongoDB {

    pub async fn ping(&self) {
        let command = doc! {
            COMMAND_PING: 1
        };
        if let Err(error) = self.database.run_command(command, None).await {
            mongodb_panic!("Ping MongoDB Server Error with stacktrace\n{}", error)
        }
    }

}