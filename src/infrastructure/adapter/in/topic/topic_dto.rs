use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TopicDto {
    pub topic: String
}