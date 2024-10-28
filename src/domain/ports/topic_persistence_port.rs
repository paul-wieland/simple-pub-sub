use std::error::Error;
use crate::domain::model::topic::Topic;

pub trait TopicPersistencePort{

    async fn create_topic(&self, topic: Topic) -> Result<(), Box<dyn Error>>;

    async fn find_topic(&self, project: &String, topic: &String) -> Result<Option<Topic>, Box<dyn Error>>;

}