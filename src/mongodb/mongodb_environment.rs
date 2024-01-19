use crate::environment::Environment;

const ENVIRONMENT_MONGODB: &str = "MONGODB";
const ENVIRONMENT_MONGODB_HOST: &str = "HOST";
const ENVIRONMENT_MONGODB_PORT: &str = "PORT";
const ENVIRONMENT_MONGODB_SOURCE: &str = "SOURCE";
const ENVIRONMENT_MONGODB_USERNAME: &str = "USERNAME";
const ENVIRONMENT_MONGODB_PASSWORD: &str = "PASSWORD";
const ENVIRONMENT_MONGODB_DATABASE: &str = "DATABASE";

pub trait MongoDBEnvironment {

    fn mongodb(&self, field: &str) -> Option<String>;

    fn mongodb_host(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_HOST)
    }

    fn mongodb_port(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_PORT)
    }

    fn mongodb_source(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_SOURCE)
    }

    fn mongodb_username(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_USERNAME)
    }

    fn mongodb_password(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_PASSWORD)
    }

    fn mongodb_database(&self) -> Option<String> {
        self.mongodb(ENVIRONMENT_MONGODB_DATABASE)
    }

}

impl MongoDBEnvironment for Environment {
    fn mongodb(&self, field: &str) -> Option<String> {
        self.variable(ENVIRONMENT_MONGODB, field)
    }
}