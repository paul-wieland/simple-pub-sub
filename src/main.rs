mod infrastructure;
mod domain;

use std::error::Error;
use crate::infrastructure::adapter::config::http_server_config::HttpServerConfig;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{


    HttpServerConfig::run().await
}
