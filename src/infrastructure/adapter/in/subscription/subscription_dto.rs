use serde::{Deserialize, Serialize};
use crate::domain::model::subscription::Subscription;

#[derive(Deserialize, Serialize)]
pub struct SubscriptionDto {
    pub subscription: String
}

//-------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct SubscriptionsResponseDto {
    pub subscriptions: Vec<SubscriptionResponseDto>,
}

//-------------------------------------------------------------------

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscriptionResponseDto {
    pub project: String,
    pub topic: String,
    pub subscription: String
}

//-------------------------------------------------------------------