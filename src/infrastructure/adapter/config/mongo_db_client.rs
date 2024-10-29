use std::error::Error;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Collection};

pub struct MongoDbClient{
    client: Client,
    database_name: String
}

impl MongoDbClient {

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        let connection_string: String = "mongodb://root:root@127.0.0.1:27017/?maxPoolSize=20&w=majority".into();
        let database_name = "simple-pub-sub".into();


        let mut client_options = Self::initialize_client_options(&connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        match Client::with_options(client_options) {
            Ok(client) => { Ok(Self{ client, database_name }) }
            Err(_) => { Err("Could not initialize MongoDB client".into()) }
        }
    }

    pub fn collection<T: Send + Sync>(&self, collection_name: &str) -> Collection<T>{
        self.client.database(&self.database_name).collection(collection_name)
    }

    async fn initialize_client_options(connection_string: &str) -> Result<ClientOptions, Box<dyn Error>> {
        match ClientOptions::parse(connection_string).await {
            Ok(client_options) => { Ok(client_options) }
            Err(_) => {Err(format!("Could not initialize MongoDB connection from connection string: {}", connection_string).into())}
        }
    }

}