use std::error::Error;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::out::subscription::subscription_persistence_adapter::SubscriptionPersistenceAdapter;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;
use crate::infrastructure::adapter::r#in::subscription::subscription_api::create_subscription;
use crate::infrastructure::adapter::r#in::topic::topic_api::create_topic;

pub struct HttpServerConfig{

}

impl HttpServerConfig{

    pub async fn run() -> Result<(), Box<dyn Error>> {

        // Setup Topic UseCase

        let create_topic_use_case =
            Arc::new(
                CreateTopicUseCase::new(Box::new(TopicPersistenceAdapter::new().await?)));

        // Setup Subscription UseCase
        let subscription_persistence_adapter = SubscriptionPersistenceAdapter::new().await?;
        let create_subscription_use_case =
            Arc::new(
                CreateSubscriptionUseCase::new(
                    Box::new(subscription_persistence_adapter),
                    Box::new(TopicPersistenceAdapter::new().await?))
            );

        HttpServer::new(move || {
            App::new()
                // Topic
                .app_data(web::Data::new(create_topic_use_case.clone()))
                .service(create_topic)
                // Subscription
                .app_data(web::Data::new(create_subscription_use_case.clone()))
                .service(create_subscription)
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
            .map(|_| ())
            .map_err(|_| "Error when executing HTTP server".into())
    }

}