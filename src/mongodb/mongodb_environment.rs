use crate::environment::Environment;

const ENVIRONMENT_MONGODB: &str = "MONGODB";
const ENVIRONMENT_MONGODB_HOST: &str = "HOST";
const ENVIRONMENT_MONGODB_PORT: &str = "PORT";
const ENVIRONMENT_MONGODB_SOURCE: &str = "SOURCE";
const ENVIRONMENT_MONGODB_USERNAME: &str = "USERNAME";
const ENVIRONMENT_MONGODB_PASSWORD: &str = "PASSWORD";
const ENVIRONMENT_MONGODB_DATABASE: &str = "DATABASE";

pub trait MongoDBEnvironment {

    fn mongodb(&mut self, field: &str) -> Option<String>;

    fn mongodb_host(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_HOST)
    }

    fn mongodb_port(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_PORT)
    }

    fn mongodb_source(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_SOURCE)
    }

    fn mongodb_username(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_USERNAME)
    }

    fn mongodb_password(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_PASSWORD)
    }

    fn mongodb_database(&mut self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_DATABASE)
    }

}

impl MongoDBEnvironment for Environment {
    fn mongodb(&mut self, field: &str) -> Option<String> {
        self.take_variable(ENVIRONMENT_MONGODB, field)
    }
}