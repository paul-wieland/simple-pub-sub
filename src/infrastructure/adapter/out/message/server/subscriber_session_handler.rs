use std::error::Error;
use std::fmt::format;
use std::io;
use std::sync::Arc;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split, ReadHalf, WriteHalf, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use crate::domain::model::topic::Topic;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::{MessageCreatedNotificationAdapter, MessageResponseDto};
use crate::infrastructure::adapter::out::message::server::subscriber::{Subscriber, SubscriptionData};
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
pub struct SubscriptionDataDto {
    pub project: String,
    pub topic: String,
    pub subscription: String,
}

#[derive(Debug)]
enum SubscriberHandlerError{
    ReadError,
    WriteError
}

pub struct SubscriberSessionHandler{
    reader: Arc<Mutex<BufReader<ReadHalf<TcpStream>>>>,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
    session_id: String,
    message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>,
    subscription_data: Option<SubscriptionData>,
    subscription: Option<String>
}

impl SubscriberSessionHandler{

    pub fn new(tcp_stream: TcpStream, message_created_notification_adapter: Arc<MessageCreatedNotificationAdapter>) -> Self{
        let session_id = Uuid::new_v4().to_string();
        info!("[Session {}] Session created", &session_id);
        let (reader, writer) = split(tcp_stream);
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(reader))),
            writer: Arc::new(Mutex::new(writer)),
            session_id,
            message_created_notification_adapter,
            subscription_data: None,
            subscription: None
        }
    }

    pub async fn start(&mut self){

        if let Err(_) = self.send_session_started_message().await.map_err(|_| { SubscriberHandlerError::WriteError }){
            return
        }

        let (sender, mut receiver) = mpsc::channel::<MessageResponseDto>(100);
        let mut buffer = String::new();


        match self.initialize_connection(sender).await{
                Ok(subscription_data) => {
                    self.subscription_data = Some(subscription_data.clone());
                    self.send_session_initialize(subscription_data.clone()).await.expect("");
                    subscription_data
                }
                Err(_) => {
                    self.send_session_not_initialize().await.expect("");
                    return
                }
            };


        loop {
            select! {
                //--------------------------------------------------------------------------------
                Some(message) = receiver.recv() => {
                let response = serde_json::to_string(&message).unwrap() + "\n";
                match self.send_message_to_client(response).await {
                        Err(_) => { break }
                        _ => {}
                    }
                }
                //--------------------------------------------------------------------------------
                client_message = self.read_message_from_client(&mut buffer) => {
                    match client_message{
                    Ok(_) => { println!("received client message") }
                    Err(_) => {
                            error!("[Session {}] Session closed due to read line error", &self.session_id);
                            break
                        }
                    }
                }
                //--------------------------------------------------------------------------------
            }
        }
        info!("Destroying session {}", &self.session_id);
        self.message_created_notification_adapter.deregister(self.subscription_data.clone(), &self.session_id).await;
    }



    async fn initialize_connection(&self, sender: Sender<MessageResponseDto>) -> Result<SubscriptionData, SubscriberHandlerError>{

        // Try read data from client
        let mut buffer = String::new();
        match self.read_message_from_client(&mut buffer).await {
            Err(_) => {
                Err(SubscriberHandlerError::ReadError)
            }
            _ => { Ok(()) }
        }?;

        // Try to parse client data
        let subscription_data = match serde_json::from_str::<SubscriptionDataDto>(buffer.trim()){
            Ok(subscription_data_dto) => {
                Ok(SubscriptionData{
                    project: subscription_data_dto.project,
                    topic: subscription_data_dto.topic,
                    subscription: subscription_data_dto.subscription,
                })
            }
            Err(_) => {
                Err(SubscriberHandlerError::ReadError)
            }
        }?;

        // Register client with valid subscriber data
        let subscriber = Subscriber::new(
            sender,
            subscription_data.clone(),
            self.session_id.clone()
        );

        self.message_created_notification_adapter.register(subscriber).await;

        Ok(subscription_data)
    }

    async fn send_session_not_initialize(&self) -> Result<(), SubscriberHandlerError>{

        let log_message = format!("[Session {}] Could not initialized session. Closing connection", &self.session_id);
        info!("{}", log_message);

        let message = SubscriberSessionResponse {
            status: SubscriberSessionStatus::SubscriberSessionClosed,
            session_id: self.session_id.clone(),
            message: "Could not initialize session. Closing connection".to_string(),
        };

        let message = serde_json::to_string(&message).unwrap() + "\n";
        self.send_message_to_client(message).await
    }

    async fn send_session_initialize(&self, subscription_data: SubscriptionData) -> Result<(), SubscriberHandlerError>{

        let log_message = format!(
            "[Session {}] Initialized session with project {} and topic {} and subscription {}",
            &self.session_id,
            &subscription_data.project,
            &subscription_data.topic,
            &subscription_data.subscription);
        info!("{}", log_message);

        let message = SubscriberSessionResponse {
            status: SubscriberSessionStatus::SubscriberSessionInitialized,
            session_id: self.session_id.clone(),
            message: format!("Initialized session with project {} and topic {} and subscription {}",
                             subscription_data.project,
                             subscription_data.topic,
                             subscription_data.subscription),
        };
        let message = serde_json::to_string(&message).unwrap() + "\n";
        self.send_message_to_client(message).await
    }

    /*
        Send session started message to the client
    */
    async fn send_session_started_message(&mut self) -> Result<(), SubscriberHandlerError>{

        info!("[Session {}] Subscriber session has started", &self.session_id);
        let r = SubscriberSessionResponse{
            status: SubscriberSessionStatus::SubscriberSessionStarted,
            session_id: self.session_id.clone(),
            message: "Session has started".into(),
        };
        let message = serde_json::to_string(&r).unwrap() + "\n";
        match self.send_message_to_client(message).await {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(SubscriberHandlerError::WriteError) }
        }
    }

    /*
        Send String data to client
     */
    async fn send_message_to_client(&self, data: String) -> Result<(), SubscriberHandlerError>{
        let mut writer = self.writer.lock().await;
        writer.write_all(data.as_bytes()).await.map_err(|_| { SubscriberHandlerError::WriteError })?;
        writer.flush().await.map_err(|_| { SubscriberHandlerError::WriteError })?;
        Ok(())
    }

    async fn read_message_from_client(&self, buffer: &mut String) -> io::Result<usize> {
        let mut reader = self.reader.lock().await;
        reader.read_line(buffer).await
    }
}