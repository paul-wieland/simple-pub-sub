use std::error::Error;
use async_trait::async_trait;
use crate::domain::model::topic::Topic;
use crate::domain::ports::topic_persistence_port::TopicPersistencePort;
use crate::infrastructure::adapter::out::topic::topic_repository::TopicRepository;

pub struct TopicPersistenceAdapter{
    topic_repository: TopicRepository
}

impl TopicPersistenceAdapter{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self{ topic_repository: TopicRepository::new().await? })
    }

}

#[async_trait]
impl TopicPersistencePort for TopicPersistenceAdapter {
    async fn create_topic(&self, topic: Topic) -> Result<(), Box<dyn Error>> {
        self.topic_repository.create_topic(topic).await
    }

    async fn find_topic(&self, project: &str, topic: &str) -> Result<Option<Topic>, Box<dyn Error>> {
        self.topic_repository.find_topic(project, topic).await
    }
}