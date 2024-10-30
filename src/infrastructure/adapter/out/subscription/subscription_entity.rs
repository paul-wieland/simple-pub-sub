use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::domain::model::subscription::Subscription;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionEntity{
    pub project: String,
    pub topic: String,
    pub subscription: String,
    pub created: DateTime,
}

impl From<Subscription> for SubscriptionEntity{
    fn from(value: Subscription) -> Self {
        SubscriptionEntity{
            project: value.project.clone(),
            topic: value.topic.clone(),
            subscription: value.subscription.clone(),
            created: DateTime::now(),
        }
    }
}

impl From<SubscriptionEntity> for Subscription{
    fn from(value: SubscriptionEntity) -> Self {
        Subscription{
            project: value.project,
            topic: value.topic,
            subscription: value.subscription,
        }
    }
}