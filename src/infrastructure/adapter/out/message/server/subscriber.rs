use tokio::sync::mpsc;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageResponseDto;

pub struct Subscriber{
    pub sender: mpsc::Sender<MessageResponseDto>,
    pub project_topic_subscription: String
}

impl Subscriber{

    pub fn new(
        sender: mpsc::Sender<MessageResponseDto>,
        project: String,
        topic: String,
        subscription: String) -> Self{

        Self{
            sender,
            project_topic_subscription: format!("{}_{}_{}",project, topic, subscription )
        }

    }

}