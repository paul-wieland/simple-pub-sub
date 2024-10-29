mod infrastructure;
mod domain;

use std::error::Error;
use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use crate::domain::model::topic::Topic;
use crate::domain::ports::topic_persistence_port::TopicPersistencePort;
use crate::infrastructure::adapter::out::topic_persistence_adapter::TopicPersistenceAdapter;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    _id: i32,
    title: String,
    author: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {



    let topic = Topic { project: "test-project".to_string(), topic: "v33.user-update".to_string() };

    let topic_persistence_adapter = TopicPersistenceAdapter::new().await?;
    topic_persistence_adapter.create_topic(topic).await?;

    let result = topic_persistence_adapter.find_topic("test-project", "v5.user-update").await?;

    match result {
      Some(t) => {
          println!("Result: project={}, topic={}", &t.project, &t.topic)
      }
        None => {
            println!("No Result available")
        }
    };


    Ok(())
}
