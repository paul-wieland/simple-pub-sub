use std::error::Error;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;
use crate::infrastructure::adapter::r#in::topic::topic_api::create_topic;

pub struct HttpServerConfig{

}

impl HttpServerConfig{

    pub async fn run() -> Result<(), Box<dyn Error>> {

        // Setup Topic UseCase
        let topic_persistence_adapter = TopicPersistenceAdapter::new().await?;
        let create_topic_use_case = Arc::new(CreateTopicUseCase::new(Box::new(topic_persistence_adapter)));

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(create_topic_use_case.clone()))
                .service(create_topic)
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
            .map(|_| ())
            .map_err(|_| "Error when executing HTTP server".into())
    }

}