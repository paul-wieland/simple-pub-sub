mod infrastructure;
mod domain;

use std::error::Error;
use simple_pub_sub::run;

#[tokio::main]
async fn main() -> Result<(), String>{
    run().await
}

