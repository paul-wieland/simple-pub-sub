use std::error::Error;
use std::sync::Arc;
use log::info;
use tokio::net::TcpListener;
use tokio::task;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::message::server::subscriber_session_handler::SubscriberSessionHandler;

pub struct SubscriberServer{
    message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>
}

impl SubscriberServer{

    pub fn new(message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>) -> Self{
        Self { message_created_notification_adapter }
    }

    pub async fn start(&self, address: &str) -> Result<(), Box<dyn Error>>{
        let listener = TcpListener::bind(address).await?;
        info!("Subscriber server is active on {}", address);

        loop{
            let notification_adapter = Arc::clone(&self.message_created_notification_adapter);
            let (tcp_stream, _) = listener.accept().await?;
            task::spawn( async move {
                SubscriberSessionHandler::new(
                    tcp_stream,
                    notification_adapter
                ).start().await;
            });
        }
    }
}