use log::{error, info};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct SubscriberSessionHandler{
    tcp_stream: TcpStream,
    session_id: String
}

impl SubscriberSessionHandler{

    pub fn new(tcp_stream: TcpStream) -> Self{
        let session_id = Uuid::new_v4().to_string();
        info!("[Session {}] Session created", &session_id);
        Self { tcp_stream, session_id }
    }

    pub async fn start(&mut self){
        let (split_reader, mut split_writer) = self.tcp_stream.split();
        let mut reader = BufReader::new(split_reader);

        loop{

            let message = "message".to_string();

            split_writer.write_all(message.as_bytes()).await.expect("");


        }
    }
}