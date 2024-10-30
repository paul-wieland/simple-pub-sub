use std::error::Error;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::topic::Topic;
use crate::domain::ports::topic_persistence_port::TopicPersistencePort;

pub struct CreateTopicUseCase {
    topic_persistence_port: Box<dyn TopicPersistencePort>
}


impl CreateTopicUseCase {

    pub fn new(topic_persistence_port: Box<dyn TopicPersistencePort>) -> Self{
        Self { topic_persistence_port }
    }

    pub async fn create_topic(&self, topic: Topic) -> Result<(), ServiceError>{
        self.topic_persistence_port.create_topic(topic).await
    }

}