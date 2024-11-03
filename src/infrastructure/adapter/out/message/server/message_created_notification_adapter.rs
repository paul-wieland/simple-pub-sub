use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::info;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::infrastructure::adapter::out::message::server::subscriber::Subscriber;

type ProjectTopicSubscription = String;

#[derive(Serialize)]
pub struct MessageResponseDto {
    pub project: String,
    pub topic: String,
    pub subscription: String,
    pub message_id: String,
    pub data: String,
    pub publish_time: DateTime<Utc>,
    pub attributes: Option<HashMap<String, Value>>,
    pub acknowledged: bool
}


pub struct MessageCreatedNotificationAdapter{
    channels: Mutex<HashMap<ProjectTopicSubscription, Vec<Subscriber>>>,
}

impl MessageCreatedNotificationAdapter{

    pub fn new() -> Self{
        Self{ channels: Mutex::new(HashMap::new()) }
    }

    pub async fn register(&self, subscriber: Subscriber) {
        let mut channels = self.channels.lock().await;
        println!("Register subscriber {}", &subscriber.project_topic_subscription);

        match channels.get(&subscriber.project_topic_subscription){
            None => {
                let mut vec = Vec::new();
                let id = subscriber.project_topic_subscription.clone();
                vec.push(subscriber);
                info!("Inserted {}", &id);
                channels.insert(id, vec);
            }
            Some(_) => {
                info!("Found something")
            }
        }
    }

    pub async fn publish_message(&self, pub_sub_message: PubSubMessage){
        let mut channels = self.channels.lock().await;

        let project = pub_sub_message.project.clone();
        let topic = pub_sub_message.topic.clone();
        let subscription = pub_sub_message.subscription.clone().unwrap();
        let key = format!("{}_{}_{}", project, topic, subscription);

        match channels.get(&key){
            None => {
                info!("no subscriber found")
            }
            Some(subscribers) => {

                let response = MessageResponseDto{
                    project: pub_sub_message.project,
                    topic: pub_sub_message.topic,
                    subscription: pub_sub_message.subscription.unwrap(),
                    message_id: pub_sub_message.message_id,
                    data: pub_sub_message.data,
                    publish_time: pub_sub_message.publish_time,
                    attributes: pub_sub_message.attributes,
                    acknowledged: pub_sub_message.acknowledged,
                };

                subscribers.first().unwrap().sender.send(response).await.expect("");
            }
        }
    }

}