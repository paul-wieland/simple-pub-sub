use std::collections::HashMap;
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::model::pub_sub_message::PubSubMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity{
    pub project: String,
    pub topic: String,
    pub subscription: String,
    pub data: String,
    pub message_id: String,
    pub publish_time: BsonDateTime,
    pub attributes: Option<HashMap<String, Value>>,
    pub created: BsonDateTime,
    pub acknowledged: bool
}

impl From<PubSubMessage> for MessageEntity{
    fn from(value: PubSubMessage) -> Self {
        MessageEntity{
            project: value.project,
            topic: value.topic,
            subscription: value.subscription.unwrap(),
            data: value.data,
            message_id: value.message_id,
            publish_time: BsonDateTime::from(value.publish_time),
            attributes: value.attributes,
            created: BsonDateTime::now(),
            acknowledged: value.acknowledged
        }
    }
}