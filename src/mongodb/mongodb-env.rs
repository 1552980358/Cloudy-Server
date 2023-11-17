use crate::util::envvar;

const ENV_CLOUDY_MONGODB_HOST: &str = "CLOUDY_MONGODB_HOST";
pub fn server_host() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_HOST)
}

const ENV_CLOUDY_MONGODB_PORT: &str = "CLOUDY_MONGODB_PORT";
pub fn server_port() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_PORT)
}

const ENV_CLOUDY_MONGODB_SRC: &str = "CLOUDY_MONGODB_SRC";
pub fn credential_source() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_SRC)
}

const ENV_CLOUDY_MONGODB_USER: &str = "CLOUDY_MONGODB_USER";
pub fn credential_username() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_USER)
}

const ENV_CLOUDY_MONGODB_PWD: &str = "CLOUDY_MONGODB_PWD";
pub fn credential_password() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_PWD)
}

const ENV_CLOUDY_MONGODB_DB: &str = "CLOUDY_MONGODB_DB";
pub fn database() -> Option<String> {
    envvar::with_option(ENV_CLOUDY_MONGODB_DB)
}
