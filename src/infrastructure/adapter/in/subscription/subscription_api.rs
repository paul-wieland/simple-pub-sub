use std::future::Future;
use std::sync::Arc;
use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::subscription::Subscription;
use crate::domain::usecase::create_subscription_use_case::CreateSubscriptionUseCase;
use crate::domain::usecase::get_subscriptions_use_case::GetSubscriptionsUseCase;
use crate::infrastructure::adapter::r#in::subscription::subscription_dto::{SubscriptionDto, SubscriptionResponseDto, SubscriptionsResponseDto};

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

#[get("/v1/projects/{project}/topics/{topic}/subscriptions")]
pub async fn get_subscriptions(
    use_case: web::Data<Arc<GetSubscriptionsUseCase>>,
    subscription_path: web::Path<SubscriptionPath>
) -> impl Responder {


    match use_case.find_subscriptions(&subscription_path.project, &subscription_path.topic).await {
        Ok(subscriptions) => {

            let subscriptions_dto: Vec<SubscriptionResponseDto> = subscriptions.iter().map(|subscription| {
                    SubscriptionResponseDto{
                        project: subscription.project.clone(),
                        topic: subscription.topic.clone(),
                        subscription: subscription.subscription.clone(),
                    }
                })
                .collect();

            HttpResponse::Ok().json(
                SubscriptionsResponseDto{
                    subscriptions: subscriptions_dto
                }
            )
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }

    // HttpResponse::Ok()

    //
    // let subscription = Subscription{
    //     project: subscription_path.project.clone(),
    //     topic: subscription_path.topic.clone(),
    //     subscription: subscription_dto.subscription.clone()
    // };
    //
    // match use_case.create_subscription(subscription).await{
    //     Ok(_) => { HttpResponse::Created().finish() }
    //     Err(ServiceError::ResourceNotExists(message)) => { HttpResponse::BadRequest().json(message) }
    //     Err(ServiceError::ResourceExists) => { HttpResponse::Conflict()
    //         .json(format!("Subscription `{}` already exists in project `{}` and topic `{}`",
    //                       subscription_dto.subscription, subscription_path.project, subscription_path.topic))
    //     }
    //     _ => { HttpResponse::InternalServerError().finish() }
    // }
}