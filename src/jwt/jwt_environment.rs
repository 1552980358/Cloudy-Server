use crate::environment::Environment;

use crate::jwt::jwt_panic;

const ENVIRONMENT_JWT: &str = "JWT";
const ENVIRONMENT_JWT_SECRET: &str = "SECRET";
const ENVIRONMENT_JWT_ALGORITHM: &str = "ALGORITHM";

pub trait JWTEnvironment {

    fn jwt(&mut self, field: &str) -> Option<String>;

    fn jwt_secret(&mut self) -> String {
        self.jwt(ENVIRONMENT_JWT_SECRET)
            .unwrap_or_else(|| jwt_panic!("Secret required!"))
    }

    fn jwt_algorithm(&mut self) -> Option<String> {
        self.jwt(ENVIRONMENT_JWT_ALGORITHM)
    }

}

impl JWTEnvironment for Environment {

    fn jwt(&mut self, field: &str) -> Option<String> {
        self.take_variable(ENVIRONMENT_JWT, field)
    }

}

