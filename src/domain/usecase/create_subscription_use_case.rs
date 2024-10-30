use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::ports::subscription_persistence_port::SubscriptionPersistencePort;

pub struct CreateSubscriptionUseCase {
    subscription_persistence_port: Box<dyn SubscriptionPersistencePort>
}


impl CreateSubscriptionUseCase {

    pub fn new(subscription_persistence_port: Box<dyn SubscriptionPersistencePort>) -> Self{
        Self { subscription_persistence_port }
    }

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<(), ServiceError>{
        self.subscription_persistence_port.create_subscription(subscription).await
    }

}