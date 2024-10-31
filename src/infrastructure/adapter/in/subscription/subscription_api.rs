use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder, web};
use serde::Deserialize;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::infrastructure::adapter::r#in::subscription::subscription_dto::SubscriptionDto;

#[derive(Deserialize)]
struct SubscriptionPath{
    project: String,
    topic: String
}

#[post("/v1/projects/{project}/topics/{topic}/subscriptions")]
pub async fn create_subscription(
    subscription_dto: web::Json<SubscriptionDto>,
    use_case: web::Data<Arc<CreateSubscriptionUseCase>>,
    subscription_path: web::Path<SubscriptionPath>
) -> impl Responder {

    let subscription = Subscription{
        project: subscription_path.project.clone(),
        topic: subscription_path.topic.clone(),
        subscription: subscription_dto.subscription.clone()
    };

    match use_case.create_subscription(subscription).await{
        Ok(_) => { HttpResponse::Created().finish() }
        Err(ServiceError::ResourceNotExists(message)) => { HttpResponse::BadRequest().json(message) }
        Err(ServiceError::ResourceExists) => { HttpResponse::Conflict()
            .json(format!("Subscription `{}` already exists in project `{}` and topic `{}`",
                          subscription_dto.subscription, subscription_path.project, subscription_path.topic))
        }
        _ => { HttpResponse::InternalServerError().finish() }
    }
}