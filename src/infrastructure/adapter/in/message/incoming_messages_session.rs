use std::sync::Arc;
use actix_web::body::MessageBody;
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
    Ok,
    Error
}

#[derive(Debug, Serialize)]
struct SessionResponse{
    status: SessionStatus,
    session_id: String,
    message: String
}

pub struct IncomingMessagesSession {
    session_id: String,
    tcp_stream: TcpStream,
    create_message_use_case: Arc<CreateMessageUseCase>
}

impl IncomingMessagesSession {

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
            status: SessionStatus::Ok,
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
                match serde_json::from_str::<ProjectTopicInitDto>(buffer.trim()){
                    Ok(project_topic_init_dto) => {
                        is_initialized = true;
                        project = Some(project_topic_init_dto.project);
                        topic = Some(project_topic_init_dto.topic);

                        let message = format!(
                            "[Session {}] Initialized session with project {} and topic {}",
                            &self.session_id,
                            project.clone().unwrap(),
                            topic.clone().unwrap());
                        // TODO: send formatted session initialized message to client
                        info!("{}", message)
                    }
                    Err(_) => {
                        let message = format!("[Session {}] Could not initialize session. Closing connection", &self.session_id);
                        // TODO: send formatted session closed message to client
                        split_writer.write_all(message.as_bytes()).await.expect("");
                        self.tcp_stream.shutdown().await.expect("");
                        error!("{}" ,message);
                        break
                    }
                }

            }else{
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
                                let message = format!("[Session {}] Successfully published message", &self.session_id);
                                info!("{}", &message)
                                // TODO: Send publish success to client
                            }
                            Err(_) => {
                                // TODO: Send publish message error to client
                            }
                        };
                    }
                    Err(_) => {
                        // TODO: Send deserialization error to client
                    }
                }
            }
        }
        debug!("Destroying session {}", &self.session_id)
    }
}

