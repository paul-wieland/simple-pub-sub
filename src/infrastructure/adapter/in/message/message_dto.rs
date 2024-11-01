use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::model::pub_sub_message::PubSubMessage;

#[derive(Deserialize)]
pub struct MessageRequestDto {
        pub data: String,
        pub attributes: Option<HashMap<String, Value>>,
}

#[derive(Serialize)]
pub struct MessageResponseDto {
        pub data: String,
        pub attributes: Option<HashMap<String, Value>>,
        pub message_id: String,
        pub publish_time: String
}

impl From<PubSubMessage> for MessageResponseDto{
        fn from(value: PubSubMessage) -> Self {
                MessageResponseDto{
                        data: value.data,
                        attributes: value.attributes,
                        message_id: value.message_id,
                        publish_time: value.publish_time.to_string(),
                }
        }
}