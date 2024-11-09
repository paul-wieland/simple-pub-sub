use std::future::Future;
use std::sync::Arc;
use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::header::{ContentType, HeaderName, HeaderValue};
use serde::Deserialize;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::topic::Topic;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::domain::usecase::get_topics_use_case::GetTopicsUseCase;
use crate::infrastructure::adapter::r#in::topic::topic_dto::{TopicRequestDto, TopicResponseDto, TopicsResponseDto};

#[derive(Deserialize)]
struct ProjectPath{
    project: String
}

#[post("/v1/projects/{project}/topics")]
pub async fn create_topic(
    topic_dto: web::Json<TopicRequestDto>,
    use_case: web::Data<Arc<CreateTopicUseCase>>,
    project_path: web::Path<ProjectPath>
) -> impl Responder {

    let topic = Topic{
        project: project_path.project.clone(),
        topic: topic_dto.topic.clone()
    };

    match use_case.create_topic(topic).await{
        Ok(_) => { HttpResponse::Created().finish() }
        Err(ServiceError::ResourceExists) => { HttpResponse::Conflict()
            .json(format!("Topic `{}` already exists in project `{}`", topic_dto.topic, project_path.project))
        }
        _ => { HttpResponse::InternalServerError().finish() }
    }
}

#[get("/v1/projects/{project}/topics")]
pub async fn get_topics(
    use_case: web::Data<Arc<GetTopicsUseCase>>,
    project_path: web::Path<ProjectPath>
) -> impl Responder {


    match use_case.find_topics(&project_path.project).await {
        Ok(topics) => {
            let topics_dto_vec: Vec<TopicResponseDto> = topics.iter().map(|topic| {
                TopicResponseDto{
                    project: topic.project.clone(),
                    topic: topic.topic.clone()
                }
            }).collect();

            let topics_dto = TopicsResponseDto{
                topics: topics_dto_vec
            };

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(topics_dto)
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}