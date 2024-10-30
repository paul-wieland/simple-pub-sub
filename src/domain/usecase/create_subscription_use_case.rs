use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::ports::subscription_persistence_port::SubscriptionPersistencePort;
use crate::domain::ports::topic_persistence_port::TopicPersistencePort;

pub struct CreateSubscriptionUseCase {
    subscription_persistence_port: Box<dyn SubscriptionPersistencePort>,
    topic_persistence_port: Box<dyn TopicPersistencePort>
}


impl CreateSubscriptionUseCase {

    pub fn new(subscription_persistence_port: Box<dyn SubscriptionPersistencePort>,
               topic_persistence_port: Box<dyn TopicPersistencePort>) -> Self{
        Self { subscription_persistence_port, topic_persistence_port }
    }

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<(), ServiceError>{

        if let Ok(None) = self.topic_persistence_port.find_topic(&subscription.project, &subscription.topic).await{
            return Err(ServiceError::ResourceNotExists(
                format!("Cannot create subscription `{}`. Topic `{}` in project `{} does not exist`",
                        subscription.subscription,
                        subscription.topic,
                        subscription.project)
            ))
        }

        self.subscription_persistence_port.create_subscription(subscription).await
    }

}