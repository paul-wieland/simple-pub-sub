use futures::SinkExt;
use log::debug;
use tokio::sync::mpsc;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageResponseDto;

pub struct Subscribers{
    subscribers: Vec<Subscriber>,
    round_robin_index: usize
}


impl Subscribers{

    pub fn new() -> Self{
        Self { subscribers: Vec::new(), round_robin_index: 0 }
    }

    pub async fn send_message(&mut self, message: MessageResponseDto){
        if self.subscribers.is_empty() {
            self.round_robin_index = 0;
            return
        }
        let subscriber = self.subscribers.get(self.round_robin_index).unwrap();
        subscriber.send_message(message).await;
        debug!("Send message to subscriber with index {} and session_id {}", self.round_robin_index, &subscriber.session_id);
        self.round_robin_index = (self.round_robin_index + 1) % self.subscribers.len();
    }

    fn is_not_empty(&self) -> bool{
        !self.subscribers.is_empty()
    }

    fn is_empty(&self) -> bool{
        self.subscribers.is_empty()
    }

    pub fn add_subscriber(&mut self, subscriber: Subscriber){
        self.subscribers.push(subscriber);
        debug!("Number of subscribers {}", self.subscribers.len());
    }

    pub fn remove_subscriber(&mut self, subscriber_id: &str){
        self.subscribers.retain(|subscriber| subscriber.session_id != subscriber_id);
        if self.is_empty() {
            self.round_robin_index = 0;
        }else{
            self.round_robin_index = self.round_robin_index % self.subscribers.len();
        }
        debug!("New round robin index {}", self.round_robin_index);
        debug!("Number of subscribers {}", self.subscribers.len());
    }

}


// ------------------------------------------------------------------------------------------------

pub struct Subscriber{
    pub sender: mpsc::Sender<MessageResponseDto>,
    pub subscription_data: SubscriptionData,
    pub session_id: String
}

impl Subscriber{

    pub fn new(
        sender: mpsc::Sender<MessageResponseDto>,
        subscription_data: SubscriptionData,
        session_id: String) -> Self{

        Self{ sender, subscription_data, session_id }
    }

    pub async fn send_message(&self, message: MessageResponseDto){
        &self.sender.send(message).await;
    }

}

// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct SubscriptionData{
    pub project: String,
    pub topic: String,
    pub subscription: String
}

impl SubscriptionData{

    pub fn full_subscription_key(&self) -> String {
        format!("{}_{}_{}", self.project, self.topic, self.subscription)
    }

}