use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::{debug};
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::infrastructure::adapter::out::message::server::subscriber::{Subscriber, Subscribers, SubscriptionData};

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
    channels: Mutex<HashMap<ProjectTopicSubscription, Subscribers>>,
}

impl MessageCreatedNotificationAdapter{

    pub fn new() -> Self{
        Self{ channels: Mutex::new(HashMap::new()) }
    }

    pub async fn register(&self, subscriber: Subscriber) {
        let mut channels = self.channels.lock().await;

        match channels.get_mut(&subscriber.subscription_data.full_subscription_key()){
            None => {
                let subscriber_group = subscriber.subscription_data.full_subscription_key();
                let mut subscribers = Subscribers::new();
                subscribers.add_subscriber(subscriber);
                channels.insert(subscriber_group, subscribers);
            }
            Some(subscribers) => {
                subscribers.add_subscriber(subscriber);
            }
        }
    }

    pub async fn deregister(&self, subscription_data: Option<SubscriptionData>, session_id: &str){

        if let None = subscription_data{
            return
        }

        let mut channels = self.channels.lock().await;
        match channels.get_mut(&subscription_data.unwrap().full_subscription_key()){
            None => { }
            Some(subscribers) => {
                subscribers.remove_subscriber(session_id)
            }
        }
    }

    pub async fn publish_message(&self, pub_sub_message: PubSubMessage){
        let mut channels = self.channels.lock().await;

        let subscription_data = SubscriptionData{
            project: pub_sub_message.project.clone(),
            topic: pub_sub_message.topic.clone(),
            subscription: pub_sub_message.subscription.clone().unwrap(),
        };


        let full_subscription_key = subscription_data.full_subscription_key();

        match channels.get_mut(&full_subscription_key){
            None => {
                debug!("No subscribers found for {}", &full_subscription_key)
            }
            Some(subscribers) => {
                debug!("Found subscribers for {}", &full_subscription_key);
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
                subscribers.send_message(response).await;
            }
        }
    }

}