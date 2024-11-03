use std::error::Error;
use log::info;
use tokio::net::TcpListener;
use tokio::task;
use crate::infrastructure::adapter::out::message::server::subscriber_session_handler::SubscriberSessionHandler;
use crate::infrastructure::adapter::r#in::message::publisher_session_handler::PublisherSessionHandler;

pub struct SubscriberServer{


}

impl SubscriberServer{

    pub fn new() -> Self{
        Self { }
    }

    pub async fn start(&self, address: &str) -> Result<(), Box<dyn Error>>{
        let listener = TcpListener::bind(address).await?;
        info!("Subscriber server is active on {}", address);

        loop{
            let (tcp_stream, _) = listener.accept().await?;
            task::spawn( async move {
                SubscriberSessionHandler::new(tcp_stream).start().await;
            });
        }
    }
}