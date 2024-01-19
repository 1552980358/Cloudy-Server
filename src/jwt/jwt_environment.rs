use crate::environment::Environment;

use crate::jwt::jwt_panic;

const ENVIRONMENT_JWT: &str = "JWT";
const ENVIRONMENT_JWT_SECRET: &str = "SECRET";
const ENVIRONMENT_JWT_ALGORITHM: &str = "ALGORITHM";

pub trait JWTEnvironment {

    fn jwt(&self, field: &str) -> Option<String>;

    fn jwt_secret(&self) -> String {
        self.jwt(ENVIRONMENT_JWT_SECRET)
            .unwrap_or_else(|| jwt_panic!("Secret required!"))
    }

    fn jwt_algorithm(&self) -> Option<String> {
        self.jwt(ENVIRONMENT_JWT_ALGORITHM)
    }

}

impl JWTEnvironment for Environment {

    fn jwt(&self, field: &str) -> Option<String> {
        self.variable(ENVIRONMENT_JWT, field)
    }

}

