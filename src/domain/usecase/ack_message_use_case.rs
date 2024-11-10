use crate::domain::model::service_error::ServiceError;
use crate::domain::ports::message_persistence_port::MessagePersistencePort;

pub struct AckMessageUseCase {
    message_persistence_port: Box<dyn MessagePersistencePort>
}


impl AckMessageUseCase {

    pub fn new(message_persistence_port: Box<dyn MessagePersistencePort>,) -> Self{
        Self { message_persistence_port }
    }

    pub async fn ack_message(&self, project: &str, topic: &str, message_id: &str) -> Result<(), ServiceError>{
        self.message_persistence_port.ack_message(project, topic, message_id).await
    }

}