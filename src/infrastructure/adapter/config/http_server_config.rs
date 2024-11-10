use std::error::Error;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::domain::usecase::get_subscriptions_use_case::GetSubscriptionsUseCase;
use crate::domain::usecase::get_topics_use_case::GetTopicsUseCase;
use crate::infrastructure::adapter::out::message::persistence::message_persistence_adapter::MessagePersistenceAdapter;
use crate::infrastructure::adapter::out::message::server::message_created_notification_adapter::MessageCreatedNotificationAdapter;
use crate::infrastructure::adapter::out::subscription::subscription_persistence_adapter::SubscriptionPersistenceAdapter;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;
use crate::infrastructure::adapter::r#in::message::messages_api::create_message;
use crate::infrastructure::adapter::r#in::subscription::subscription_api::{create_subscription, get_subscriptions};
use crate::infrastructure::adapter::r#in::topic::topic_api::{create_topic, get_topics};

pub struct HttpServerConfig{
    create_topic_use_case: Arc<CreateTopicUseCase>,
    create_subscription_use_case: Arc<CreateSubscriptionUseCase>,
    create_message_use_case: Arc<CreateMessageUseCase>,
    get_topics_use_case: Arc<GetTopicsUseCase>,
    get_subscription_use_case: Arc<GetSubscriptionsUseCase>
}

impl HttpServerConfig{

    pub fn new(
        create_topic_use_case: Arc<CreateTopicUseCase>,
        create_subscription_use_case: Arc<CreateSubscriptionUseCase>,
        create_message_use_case: Arc<CreateMessageUseCase>,
        get_topics_use_case: Arc<GetTopicsUseCase>,
        get_subscription_use_case: Arc<GetSubscriptionsUseCase>
    ) -> Self{
        Self {
            create_topic_use_case,
            create_subscription_use_case,
            create_message_use_case,
            get_topics_use_case,
            get_subscription_use_case
        }
    }

    pub async fn start(&self, address: &str) {

        let topic_use_case = self.create_topic_use_case.clone();
        let subscription_use_case = self.create_subscription_use_case.clone();
        let message_use_case = self.create_message_use_case.clone();
        let get_topics_use_case = self.get_topics_use_case.clone();
        let get_subscription_use_case = self.get_subscription_use_case.clone();

        match HttpServer::new( move || {
            App::new()
                // Topic
                .app_data(web::Data::new(topic_use_case.clone()))
                .app_data(web::Data::new(get_topics_use_case.clone()))
                .service(create_topic)
                .service(get_topics)
                // Subscription
                .app_data(web::Data::new(subscription_use_case.clone()))
                .app_data(web::Data::new(get_subscription_use_case.clone()))
                .service(create_subscription)
                .service(get_subscriptions)
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