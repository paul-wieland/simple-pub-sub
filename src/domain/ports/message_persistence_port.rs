use async_trait::async_trait;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::domain::model::service_error::ServiceError;

#[async_trait]
pub trait MessagePersistencePort: Send + Sync{

    async fn create_message(&self,message: PubSubMessage) -> Result<(), ServiceError>;

}