use std::sync::Arc;
use actix_web::body::MessageBody;
use actix_web::cookie::Expiration::Session;
use chrono::Utc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, WriteHalf};
use uuid::Uuid;
use log::{debug, error, info};
use serde::Serialize;
use tokio::net::TcpStream;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::infrastructure::adapter::r#in::message::message_dto::{MessageRequestDto, ProjectTopicInitDto};

#[derive(Debug, Serialize)]
enum SessionStatus{
    SessionStarted,
    SessionInitialized,
    SessionClosed,
    SessionMessagePublished,
    SessionMessagePublishedError
}

#[derive(Debug, Serialize)]
struct SessionResponse{
    status: SessionStatus,
    session_id: String,
    message: String
}

pub struct PublisherSessionHandler {
    session_id: String,
    tcp_stream: TcpStream,
    create_message_use_case: Arc<CreateMessageUseCase>
}

impl PublisherSessionHandler {

    pub fn new(tcp_stream: TcpStream, create_message_use_case: Arc<CreateMessageUseCase>) -> Self{
        let session_id = Uuid::new_v4().to_string();
        info!("[Session {}] Session created", &session_id);
        Self { session_id, tcp_stream, create_message_use_case}
    }

    pub async fn start(&mut self) {
        // Split socket in read and write part
        let (split_reader, mut split_writer) = self.tcp_stream.split();

        /*
            Log and send session started to client
         */
        info!("[Session {}] Session has started", &self.session_id.clone());
        let r = SessionResponse{
            status: SessionStatus::SessionStarted,
            session_id: self.session_id.to_string(),
            message: "Session has started".into(),
        };
        let session_started_message = serde_json::to_string(&r).unwrap() + "\n";
        split_writer.write_all(session_started_message.as_bytes()).await.expect("");


        /*
            Prepare receiving messages
         */
        let mut reader = BufReader::new(split_reader);
        let mut is_initialized = false;
        let mut project: Option<String> = None;
        let mut topic: Option<String> = None;

        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer).await{
                Ok(_) => {  }
                Err(_) => {
                    error!("[Session {}] Session closed due to read line error", &self.session_id);
                    break
                }
            }

            if !is_initialized {
                /*
                    Initialize Session
                 */
                match serde_json::from_str::<ProjectTopicInitDto>(buffer.trim()){
                    Ok(project_topic_init_dto) => {
                        // Initialize the session. TODO: Verify project and topic exists
                        project = Some(project_topic_init_dto.project);
                        topic = Some(project_topic_init_dto.topic);
                        is_initialized = true;

                        // Send initialized message to client
                        let client_message = SessionResponse{
                            status: SessionStatus::SessionInitialized,
                            session_id: self.session_id.clone(),
                            message: format!("Initialized session with project {} and topic {}",
                                             project.clone().unwrap(),
                                             topic.clone().unwrap()),
                        };
                        let client_message_raw = serde_json::to_string(&client_message).unwrap() + "\n";
                        split_writer.write_all(client_message_raw.as_bytes()).await.expect("");

                        // Log initialized message
                        let log_message = format!(
                            "[Session {}] Initialized session with project {} and topic {}",
                            &self.session_id,
                            project.clone().unwrap(),
                            topic.clone().unwrap());
                        info!("{}", log_message)
                    }
                    /*
                        Handle Session Initialization Error
                     */
                    Err(_) => {
                        let error_response = SessionResponse{
                            status: SessionStatus::SessionClosed,
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
                        break
                    }
                }

            }else{
                /*
                    Handle Message flow after initialization
                 */
                match serde_json::from_str::<MessageRequestDto>(buffer.trim()){
                    Ok(message_request) => {
                        let pub_sub_message = PubSubMessage{
                            project: project.clone().unwrap(),
                            topic: topic.clone().unwrap(),
                            subscription: None,
                            message_id: Uuid::new_v4().to_string(),
                            data: message_request.data,
                            publish_time: Utc::now(),
                            attributes: message_request.attributes,
                            acknowledged: false
                        };
                        match self.create_message_use_case.create_message(pub_sub_message).await{
                            Ok(_) => {
                                let published_response = SessionResponse{
                                    status: SessionStatus::SessionMessagePublished,
                                    session_id: self.session_id.clone(),
                                    message: "Successfully published message".to_string(),
                                };
                                // Write error message to client
                                let response = serde_json::to_string(&published_response).unwrap() + "\n";
                                split_writer.write_all(response.as_bytes()).await.expect("");
                                // Log message
                                let message = format!("[Session {}] Successfully published message", &self.session_id);
                                info!("{}", &message)
                            }
                            Err(_) => {
                                let published_response = SessionResponse{
                                    status: SessionStatus::SessionMessagePublishedError,
                                    session_id: self.session_id.clone(),
                                    message: "Could not publish message. Internal Error".to_string(),
                                };
                                // Write error message to client
                                let response = serde_json::to_string(&published_response).unwrap() + "\n";
                                split_writer.write_all(response.as_bytes()).await.expect("");
                                // Log message
                                let message = format!("[Session {}] Could not publish message. Internal Error", &self.session_id);
                                info!("{}", &message)
                            }
                        };
                    }
                    /*
                        Handle message deserialization exception
                     */
                    Err(_) => {
                        let error_response = SessionResponse{
                            status: SessionStatus::SessionClosed,
                            session_id: self.session_id.clone(),
                            message: "Could not deserialize message. Closing connection".to_string(),
                        };
                        // Write error message to client
                        let error_response = serde_json::to_string(&error_response).unwrap() + "\n";
                        split_writer.write_all(error_response.as_bytes()).await.expect("");
                        // Log error message
                        let error_log = format!("[Session {}] Could not deserialize message. Closing session", &self.session_id);
                        error!("{}" ,error_log);
                        // Close connection and session
                        self.tcp_stream.shutdown().await.expect("");
                        break
                    }
                }
            }
        }
        debug!("Destroying session {}", &self.session_id)
    }
}

