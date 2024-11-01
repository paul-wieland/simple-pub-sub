use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone)]
pub struct PubSubMessage{
    pub project: String,
    pub topic: String,
    pub subscription: Option<String>,
    pub message_id: String,
    pub data: String,
    pub publish_time: DateTime<Utc>,
    pub attributes: Option<HashMap<String, Value>>,
    pub acknowledged: bool
}