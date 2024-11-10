use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::ports::subscription_persistence_port::SubscriptionPersistencePort;

pub struct GetSubscriptionsUseCase{
    subscription_persistence_port: Box<dyn SubscriptionPersistencePort>,
}

impl GetSubscriptionsUseCase{


    pub fn new(subscription_persistence_port: Box<dyn SubscriptionPersistencePort>) -> Self{
        Self { subscription_persistence_port }
    }


    pub async fn find_subscriptions(&self, project: &str, topic: &str) -> Result<Vec<Subscription>, ServiceError>{
        self.subscription_persistence_port.find_many_subscriptions(project, topic).await
    }


}