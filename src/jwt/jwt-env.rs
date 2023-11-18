use crate::jwt::self_panic;
use crate::util::envvar;

const ENV_CLOUDY_JWT_SECRET: &str = "CLOUDY_JWT_SECRET";
pub fn secret() -> String {
    envvar::with_option(ENV_CLOUDY_JWT_SECRET)
        .unwrap_or_else(|| self_panic("Secret required!"))
}

const ENV_CLOUDY_JWT_ALGORITHM: &str = "CLOUDY_JWT_ALGORITHM";
pub fn algorithm() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_JWT_ALGORITHM)
}