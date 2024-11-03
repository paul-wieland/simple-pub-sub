use std::sync::Arc;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::message::server::subscriber::Subscriber;
use crate::infrastructure::adapter::r#in::message::message_dto::ProjectTopicInitDto;

#[derive(Debug, Serialize)]
enum SubscriberSessionStatus{
    SubscriberSessionStarted,
    SubscriberSessionInitialized,
    SubscriberSessionClosed,
    SubscriberSessionMessagePublished,
    SubscriberSessionMessagePublishedError
}

#[derive(Debug, Serialize)]
struct SubscriberSessionResponse{
    status: SubscriberSessionStatus,
    session_id: String,
    message: String
}

#[derive(Deserialize)]
pub struct ProjectTopicSubscriptionInitDto {
    pub project: String,
    pub topic: String,
    pub subscription: String,
}

pub struct SubscriberSessionHandler{
    tcp_stream: TcpStream,
    session_id: String,
    message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>
}

impl SubscriberSessionHandler{

    pub fn new(tcp_stream: TcpStream, message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>) -> Self{
        let session_id = Uuid::new_v4().to_string();
        info!("[Session {}] Session created", &session_id);
        Self {
            tcp_stream,
            session_id,
            message_created_notification_adapter
        }
    }

    pub async fn start(&mut self){

        let (split_reader, mut split_writer) = self.tcp_stream.split();

        // Read initialization message: Must have project, topic, subscription
        let mut reader = BufReader::new(split_reader);

        /*
           Log and send session started to client
        */
        info!("[Session {}] Subscriber session has started", &self.session_id.clone());
        let r = SubscriberSessionResponse{
            status: SubscriberSessionStatus::SubscriberSessionStarted,
            session_id: self.session_id.to_string(),
            message: "Session has started".into(),
        };
        let session_started_message = serde_json::to_string(&r).unwrap() + "\n";
        split_writer.write_all(session_started_message.as_bytes()).await.expect("");


        let (sender, mut receiver) = mpsc::channel(100);

        /*
            Try to read initialization data
         */
        let mut buffer = String::new();
        match reader.read_line(&mut buffer).await{
            Ok(_) => {  }
            Err(_) => {
                error!("[Session {}] Session closed due to read line error", &self.session_id);
                return
            }
        }

        match serde_json::from_str::<ProjectTopicSubscriptionInitDto>(buffer.trim()){
            Ok(init_data) => {
                let subscriber = Subscriber::new(
                    sender,
                    init_data.project.clone(), init_data.topic.clone(), init_data.subscription.clone()
                );
                &self.message_created_notification_adapter.register(subscriber).await;

                // Send initialized message to client
                let message = SubscriberSessionResponse{
                    status: SubscriberSessionStatus::SubscriberSessionInitialized,
                    session_id: self.session_id.clone(),
                    message: format!("Initialized session with project {} and topic {} and subscription {}",
                                     init_data.project.clone(),
                                     init_data.topic.clone(),
                                     init_data.subscription.clone()),
                };
                let client_message_raw = serde_json::to_string(&message).unwrap() + "\n";
                split_writer.write_all(client_message_raw.as_bytes()).await.expect("");

                // Log initialized message
                let log_message = format!(
                    "[Session {}] Initialized session with project {} and topic {} and subscription {}",
                    &self.session_id,
                    init_data.project,
                    init_data.topic,
                    init_data.subscription);
                info!("{}", log_message)
            }
            Err(_) => {
                let error_response = SubscriberSessionResponse{
                    status: SubscriberSessionStatus::SubscriberSessionClosed,
                    session_id: self.session_id.clone(),
                    message: "Could not initialize session. Closing connection".to_string(),
                };
                // Write error message to client
                let error_response = serde_json::to_string(&error_response).unwrap() + "\n";
                split_writer.write_all(error_response.as_bytes()).await.expect("");
                // Log error message
                let error_log = format!("[Session {}] Could not initialize session. Closing connection", &self.session_id);
                error!("{}" ,error_log);
                // Close connection and session
                self.tcp_stream.shutdown().await.expect("");
                return
            }
        }

        loop{
            if let Some(message) = receiver.recv().await{
                let response = serde_json::to_string(&message).unwrap() + "\n";
                match split_writer.write_all(response.as_bytes()).await {
                    Ok(_) => {}
                    Err(_) => { break }
                }
            }
        }
        debug!("Destroying session {}", &self.session_id)
    }
}