use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::domain::model::topic::Topic;

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicEntity{
    pub project: String,
    pub topic: String,
    pub created: DateTime,
}

impl From<Topic> for TopicEntity{
    fn from(value: Topic) -> Self {
        TopicEntity{
            project: value.project.clone(),
            topic: value.topic.clone(),
            created: DateTime::now(),
        }
    }
}

impl From<TopicEntity> for Topic{
    fn from(value: TopicEntity) -> Self {
        Topic{
            project: value.project,
            topic: value.topic
        }
    }
}