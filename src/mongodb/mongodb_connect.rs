use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, Credential, ServerAddress, ServerApi, ServerApiVersion};

use crate::mongodb::{Mongodb, self_panic};
use crate::mongodb::mongodb_env;

impl Mongodb {

    pub async fn connect() -> Self {
        let server_host = mongodb_env::server_host();
        let server_port = mongodb_env::server_port();

        let credential_source = mongodb_env::credential_source();
        let credential_username = mongodb_env::credential_username();
        let credential_password = mongodb_env::credential_password();

        let database = mongodb_env::database();

        let storage_database = connect_storage(
            // Server Address
            server_host, server_port,
            // Credential
            credential_source, credential_username, credential_password,
            // Database
            database
        ).await;

        Mongodb::new(
        )
    }

}

async fn connect_storage(
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

    let client = connect_client(client_options);

    request_database(client, database)
}

const SERVER_ADDRESS_DEFAULT_LOCALHOST: &str = "localhost";
fn build_server_address(
    server_host: Option<String>,
    server_port: Option<String>,
) -> ServerAddress {
    let server_host = match server_host {
        Some(server_host) => { server_host }
        None => { SERVER_ADDRESS_DEFAULT_LOCALHOST.to_string() }
    };

    let server_port = match server_port {
        Some(server_port) => {
            match server_port.parse::<u16>() {
                Ok(server_port) => { Some(server_port) }
                Err(_) => { None }
            }
        }
        None => { None }
    };

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

fn connect_client(client_options: ClientOptions) -> Client {
    Client::with_options(client_options)
        .unwrap_or_else(|err| self_panic(err))
}

fn request_database(
    client: Client, database: Option<String>
) -> Database {
    match database {
        Some(database) => { client.database(&*database) }
        None => {
            client.default_database()
                .unwrap_or_else(|| self_panic("Database not found"))
        }
    }
}