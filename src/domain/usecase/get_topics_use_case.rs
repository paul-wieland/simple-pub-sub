use crate::domain::model::service_error::ServiceError;
use crate::domain::model::topic::Topic;
use crate::domain::ports::topic_persistence_port::TopicPersistencePort;

pub struct GetTopicsUseCase{
    topic_persistence_port: Box<dyn TopicPersistencePort>
}


impl GetTopicsUseCase{

    pub fn new(topic_persistence_port: Box<dyn TopicPersistencePort>) -> Self{
        Self { topic_persistence_port }
    }

    pub async fn find_topics(&self, project: &str) -> Result<Vec<Topic>, ServiceError>{
        self.topic_persistence_port.find_topics(project).await
    }

}