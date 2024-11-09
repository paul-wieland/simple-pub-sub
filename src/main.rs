mod infrastructure;
mod domain;

use std::error::Error;
use std::{env};
use std::sync::Arc;
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::infrastructure::adapter::out::message::persistence::message_persistence_adapter::MessagePersistenceAdapter;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::message::server::subscriber_server::SubscriberServer;
use crate::infrastructure::adapter::out::subscription::subscription_persistence_adapter::SubscriptionPersistenceAdapter;
use crate::infrastructure::adapter::r#in::message::publisher_server::PublisherServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    setup_logger();

    // TODO: add broadcast channel to Create Message Use Case to notify consumers -> Sender
    let message_notification_adapter = Arc::new(
        MessageCreatedNotificationAdapter::new()
    );

    /**
        Publisher Server
     */
    let create_message_use_case = Arc::new(
        CreateMessageUseCase::new(
            Box::new(MessagePersistenceAdapter::new().await?),
            Box::new(SubscriptionPersistenceAdapter::new().await?),
            Arc::clone(&message_notification_adapter)
        )
    );

    let publisher_server = tokio::spawn(async {
        PublisherServer::new(create_message_use_case)
            .start("127.0.0.1:8060")
            .await
            .expect("");
    });

    /**
        Subscriber Server
    */
    let subscriber_server = tokio::spawn( async {
        SubscriberServer::new(message_notification_adapter)
            .start("127.0.0.1:8070")
            .await
            .expect("")
    });


    /*
        TODO: Consumer broadcast channel -> Receiver
        1. On connection -> Get not acknowledged messages
        2. Listen to broadcast channel and send message to client
        3. Listen on ACKNOWLEDGE messages to ack the message at the persistence layer
     */

    tokio::join!(
        publisher_server,
        subscriber_server
    );
    Ok(())
}

pub fn setup_logger(){
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();
}
