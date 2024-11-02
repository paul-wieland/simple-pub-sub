use std::error::Error;
use std::sync::Arc;
use log::info;
use tokio::net::{TcpListener};
use tokio::task;
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::infrastructure::adapter::r#in::message::incoming_messages_session::IncomingMessagesSession;

pub struct IncomingMessagesListener {
    create_message_use_case: Arc<CreateMessageUseCase>
}

impl IncomingMessagesListener {

    pub fn new(create_message_use_case: Arc<CreateMessageUseCase>) -> Self{
        Self { create_message_use_case }
    }

    pub async fn start(&self, address: &str) -> Result<(), Box<dyn Error>>{
        let listener = TcpListener::bind(address).await?;
        info!("Message Listener for Publishers is active on {}", address);

        loop{
            let (tcp_stream, _) = listener.accept().await?;
            let cloned_use_case = Arc::clone(&self.create_message_use_case);
            task::spawn( async move {
                IncomingMessagesSession::new(tcp_stream, cloned_use_case).start().await;
            });
        }
    }
}
