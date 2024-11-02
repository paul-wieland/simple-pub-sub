use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder, web};
use chrono::{Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::model::pub_sub_message::PubSubMessage;
use crate::domain::model::topic::Topic;
use crate::domain::usecase::create_message_use_case::CreateMessageUseCase;
use crate::infrastructure::adapter::r#in::message::message_dto::{MessageRequestDto};

#[derive(Deserialize)]
struct ProjectTopicPath{
    project: String,
    topic: String
}

#[post("/v1/projects/{project}/topics/{topic}/messages")]
pub async fn create_message(
    message_request_dto: web::Json<MessageRequestDto>,
    use_case: web::Data<Arc<CreateMessageUseCase>>,
    project_topic_path: web::Path<ProjectTopicPath>
) -> impl Responder {

    let topic = Topic{
        project: project_topic_path.project.clone(),
        topic: project_topic_path.topic.clone(),
    };

    let pub_sub_message = PubSubMessage{
        project: topic.project,
        topic: topic.topic,
        subscription: None,
        message_id: Uuid::new_v4().to_string(),
        data: message_request_dto.data.clone(),
        publish_time: Utc::now(),
        attributes: message_request_dto.attributes.clone(),
        acknowledged: false
    };

    match use_case.create_message(pub_sub_message).await {
        _ => { HttpResponse::Created() }
    }
}