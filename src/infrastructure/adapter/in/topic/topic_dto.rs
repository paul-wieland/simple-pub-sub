use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TopicRequestDto {
    pub topic: String
}


#[derive(Deserialize, Serialize)]
pub struct TopicResponseDto {
    pub project: String,
    pub topic: String
}

#[derive(Deserialize, Serialize)]
pub struct TopicsResponseDto {
    pub topics: Vec<TopicResponseDto>
}