use std::error::Error;
use async_trait::async_trait;
use crate::domain::model::topic::Topic;

#[async_trait]
pub trait TopicPersistencePort{

    async fn create_topic(&self, topic: Topic) -> Result<(), Box<dyn Error>>;

    async fn find_topic(&self, project: &str, topic: &str) -> Result<Option<Topic>, Box<dyn Error>>;

}