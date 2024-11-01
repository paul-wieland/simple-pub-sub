use std::error::Error;
use async_trait::async_trait;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::domain::model::service_error::ServiceError;
use crate::domain::ports::message_persistence_port::MessagePersistencePort;
use crate::infrastructure::adapter::out::message::message_entity::MessageEntity;
use crate::infrastructure::adapter::out::message::message_repository::MessageRepository;

pub struct MessagePersistenceAdapter{
    message_repository: MessageRepository
}

impl MessagePersistenceAdapter{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self{ message_repository: MessageRepository::new().await? })
    }

}

#[async_trait]
impl MessagePersistencePort for MessagePersistenceAdapter{

    async fn create_message(&self, message: PubSubMessage) -> Result<(), ServiceError> {
        self.message_repository.create_message(MessageEntity::from(message)).await
    }
}