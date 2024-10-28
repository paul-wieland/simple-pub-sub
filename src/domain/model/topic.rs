use mongodb::bson::DateTime;
use crate::infrastructure::adapter::out::topic_entity::TopicEntity;

pub struct Topic{
    pub project: String,
    pub topic: String
}

