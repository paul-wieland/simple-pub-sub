use async_trait::async_trait;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;

#[async_trait]
pub trait SubscriptionPersistencePort : Send + Sync{

    async fn create_subscription(&self, subscription: Subscription) -> Result<(), ServiceError>;

}