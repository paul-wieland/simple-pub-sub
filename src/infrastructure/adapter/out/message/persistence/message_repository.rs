use std::error::Error;
use mongodb::Collection;
use crate::domain::model::service_error::ServiceError;
use crate::infrastructure::adapter::config::mongo_db_client::MongoDbClient;
use crate::infrastructure::adapter::out::message::persistence::message_entity::MessageEntity;

pub struct MessageRepository{
    mongodb_client: MongoDbClient,
    collection: String
}

impl MessageRepository{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        let mongodb_client = MongoDbClient::new().await?;
        let collection = "messages".into();
        Ok(Self{ mongodb_client , collection})
    }

    fn collection<T: Send + Sync>(&self) -> Collection<T>{
        self.mongodb_client.collection(&self.collection)
    }

    pub async fn create_message(&self, message_entity: MessageEntity) -> Result<(), ServiceError>{
        self.collection::<MessageEntity>().insert_one(message_entity).await
            .map(|_| { })
            .map_err(|_| ServiceError::InternalServerError)
    }

}