use std::error::Error;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::out::message::persistence::message_persistence_adapter::MessagePersistenceAdapter;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::subscription::subscription_persistence_adapter::SubscriptionPersistenceAdapter;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;
use crate::infrastructure::adapter::r#in::message::messages_api::create_message;
use crate::infrastructure::adapter::r#in::subscription::subscription_api::create_subscription;
use crate::infrastructure::adapter::r#in::topic::topic_api::create_topic;

pub struct HttpServerConfig{
    create_topic_use_case: Arc<CreateTopicUseCase>,
    create_subscription_use_case: Arc<CreateSubscriptionUseCase>,
    create_message_use_case: Arc<CreateMessageUseCase>,
}

impl HttpServerConfig{

    pub fn new(
        create_topic_use_case: Arc<CreateTopicUseCase>,
        create_subscription_use_case: Arc<CreateSubscriptionUseCase>,
        create_message_use_case: Arc<CreateMessageUseCase>
    ) -> Self{
        Self {
            create_topic_use_case,
            create_subscription_use_case,
            create_message_use_case
        }
    }

    pub async fn start(&self, address: &str) {

        let topic_use_case = self.create_topic_use_case.clone();
        let subscription_use_case = self.create_subscription_use_case.clone();
        let message_use_case = self.create_message_use_case.clone();

        match HttpServer::new( move || {
            App::new()
                // Topic
                .app_data(web::Data::new(topic_use_case.clone()))
                .service(create_topic)
                // Subscription
                .app_data(web::Data::new(subscription_use_case.clone()))
                .service(create_subscription)
                // Message
                .app_data(web::Data::new(message_use_case.clone()))
                .service(create_message)
        })
            .bind(address).unwrap_or_else(|_| { panic!("Could not start http server") })
            .run()
            .await{
            Ok(_) => {}
            Err(_) => {}
        }
    }

}