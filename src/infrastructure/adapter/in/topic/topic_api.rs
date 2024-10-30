use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder, web};
use serde::Deserialize;
use crate::domain::model::topic::Topic;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::r#in::topic::topic_dto::TopicDto;

#[derive(Deserialize)]
struct ProjectPath{
    project: String
}

#[post("/v1/projects/{project}/topics")]
pub async fn create_topic(
    topic_dto: web::Json<TopicDto>,
    use_case: web::Data<Arc<CreateTopicUseCase>>,
    project_path: web::Path<ProjectPath>
) -> impl Responder {

    let topic = Topic{
        project: project_path.project.clone(),
        topic: topic_dto.topic.clone()
    };

    match use_case.create_topic(topic).await{
        Ok(_) => { HttpResponse::Created() }
        Err(_) => { HttpResponse::BadRequest() }
    }
}