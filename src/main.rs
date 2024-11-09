mod infrastructure;
mod domain;

use std::error::Error;
use std::{env};
use std::sync::Arc;
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::config::http_server_config::HttpServerConfig;
use crate::infrastructure::adapter::out::message::persistence::message_persistence_adapter::MessagePersistenceAdapter;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::message::server::subscriber_server::SubscriberServer;
use crate::infrastructure::adapter::out::subscription::subscription_persistence_adapter::SubscriptionPersistenceAdapter;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;
use crate::infrastructure::adapter::r#in::message::publisher_server::PublisherServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    setup_logger();

    // Use Cases
    let message_notification_adapter = Arc::new(
        MessageCreatedNotificationAdapter::new()
    );


    let create_topic_use_case =
        Arc::new(
            CreateTopicUseCase::new(
                Box::new(TopicPersistenceAdapter::new().await?)));


    let create_message_use_case = Arc::new(
        CreateMessageUseCase::new(
            Box::new(MessagePersistenceAdapter::new().await?),
            Box::new(SubscriptionPersistenceAdapter::new().await?),
            Arc::clone(&message_notification_adapter)
        )
    );

    let create_subscription_use_case = Arc::new(
      CreateSubscriptionUseCase::new(
          Box::new(SubscriptionPersistenceAdapter::new().await?),
          Box::new(TopicPersistenceAdapter::new().await?)
      )
    );

    let publisher = PublisherServer::new(create_message_use_case.clone());
    let subscriber = SubscriberServer::new(message_notification_adapter.clone());
    let http = HttpServerConfig::new(
        create_topic_use_case.clone(),
        create_subscription_use_case.clone(),
        create_message_use_case.clone()
    );

    // Servers
    let publisher_server = tokio::spawn(async move {
        publisher
            .start("127.0.0.1:8060")
            .await
            .expect("");
    });

    let subscriber_server = tokio::spawn( async move {
        subscriber
            .start("127.0.0.1:8070")
            .await
            .expect("")
    });


    let http_server = tokio::spawn( async move{
        http
            .start("127.0.0.1:8080")
            .await
    });

    tokio::join!(
        publisher_server,
        subscriber_server,
        http_server
    );
    Ok(())
}

pub fn setup_logger(){
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();
}
