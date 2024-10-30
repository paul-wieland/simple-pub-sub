use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SubscriptionDto {
    pub subscription: String
}