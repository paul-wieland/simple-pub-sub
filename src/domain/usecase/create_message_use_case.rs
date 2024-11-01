use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::domain::model::service_error::ServiceError;
use crate::domain::ports::message_persistence_port::MessagePersistencePort;
use crate::domain::ports::subscription_persistence_port::SubscriptionPersistencePort;

pub struct CreateMessageUseCase{
    message_persistence_port: Box<dyn MessagePersistencePort>,
    subscription_persistence_port: Box<dyn SubscriptionPersistencePort>
}

impl CreateMessageUseCase {

    pub fn new(message_persistence_port: Box<dyn MessagePersistencePort>,
               subscription_persistence_port: Box<dyn SubscriptionPersistencePort>) -> Self{
        Self { message_persistence_port, subscription_persistence_port }
    }

    pub async  fn create_message(&self, message: PubSubMessage) -> Result<(), ServiceError>{

        let subscriptions =
            self.subscription_persistence_port.find_many_subscriptions(&message.project, &message.topic).await?;

        for subscription in subscriptions{
            let mut subscription_message = message.clone();
            // Set subscription specific values on message before persisting
            subscription_message.subscription = Some(subscription.subscription);
            subscription_message.message_id = uuid::Uuid::new_v4().to_string();
            self.message_persistence_port.create_message(subscription_message).await?;
        }

        Ok(())

    }

}