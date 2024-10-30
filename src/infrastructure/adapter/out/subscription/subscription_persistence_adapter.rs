use std::error::Error;
use async_trait::async_trait;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::ports::subscription_persistence_port::SubscriptionPersistencePort;
use crate::infrastructure::adapter::out::subscription::subscription_entity::SubscriptionEntity;
use crate::infrastructure::adapter::out::subscription::subscription_repository::SubscriptionRepository;

pub struct SubscriptionPersistenceAdapter{
    subscription_repository: SubscriptionRepository
}

impl SubscriptionPersistenceAdapter{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self{ subscription_repository: SubscriptionRepository::new().await? })
    }

}

#[async_trait]
impl SubscriptionPersistencePort for SubscriptionPersistenceAdapter {
    async fn create_subscription(&self, subscription: Subscription) -> Result<(), ServiceError> {
        self.subscription_repository.create_subscription(SubscriptionEntity::from(subscription)).await
    }

}