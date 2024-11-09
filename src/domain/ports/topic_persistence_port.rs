use std::error::Error;
use async_trait::async_trait;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::topic::Topic;

#[async_trait]
pub trait TopicPersistencePort : Send + Sync{

    async fn create_topic(&self, topic: Topic) -> Result<(), ServiceError>;

    async fn find_topic(&self, project: &str, topic: &str) -> Result<Option<Topic>, Box<dyn Error>>;


    async fn find_topics(&self, project: &str) -> Result<Vec<Topic>, ServiceError>;

}