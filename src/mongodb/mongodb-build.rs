use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, Credential, ServerAddress, ServerApi, ServerApiVersion};

use crate::mongodb::{MongoDB, self_panic};
use crate::mongodb::env;

impl MongoDB {

    pub fn connect() -> Self {
        let server_host = env::server_host();
        let server_port = env::server_port();

        let credential_source = env::credential_source();
        let credential_username = env::credential_username();
        let credential_password = env::credential_password();

        let database = env::database();

        let database = connect_database(
            // Server Address
            server_host, server_port,
            // Credential
            credential_source, credential_username, credential_password,
            // Database
            database
        );

        MongoDB::new(database)
    }

}

fn connect_database(
    server_host: Option<String>,
    server_port: Option<String>,
    credential_source: Option<String>,
    credential_username: Option<String>,
    credential_password: Option<String>,
    database: Option<String>,
) -> Database {
    let server_api = build_server_api();

    let server_address = build_server_address(
        server_host, server_port
    );

    let credential = build_credential(
        credential_source, credential_username, credential_password
    );

    let client_options = build_client_options(
        server_api, server_address, credential
    );

    let client = Client::with_options(client_options)
        .unwrap_or_else(|err| self_panic(err));

    database.map(|database| client.database(&*database))
        .unwrap_or_else(|| {
            client.default_database()
                .unwrap_or_else(|| self_panic("Database not found"))
        })
}

const SERVER_ADDRESS_DEFAULT_LOCALHOST: &str = "localhost";
fn build_server_address(
    server_host: Option<String>,
    server_port: Option<String>,
) -> ServerAddress {
    let server_host = server_host.unwrap_or_else(|| SERVER_ADDRESS_DEFAULT_LOCALHOST.to_string());

    let server_port = server_port.and_then(|origin| {
        origin.parse::<u16>().ok()
    });

    ServerAddress::Tcp {
        host: server_host,
        port: server_port
    }
}

fn build_credential(
    credential_source: Option<String>,
    credential_username: Option<String>,
    credential_password: Option<String>,
) -> Option<Credential> {
    if credential_username.is_none()
        || credential_password.is_none() {
        return None
    }

    let credential = Credential::builder()
        .username(credential_username)
        .password(credential_password)
        .source(credential_source)
        .build();
    Some(credential)
}

fn build_server_api() -> ServerApi {
    ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build()
}

fn build_client_options(
    server_api: ServerApi,
    server_address: ServerAddress,
    credential: Option<Credential>,
) -> ClientOptions {
    ClientOptions::builder()
        .server_api(server_api)
        .hosts(vec![server_address])
        .credential(credential)
        .build()
}