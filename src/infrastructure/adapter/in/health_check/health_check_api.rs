use actix_web::{get, HttpResponse, Responder};

#[get("/v1/health_check")]
pub async fn health_check(
) -> impl Responder {
    HttpResponse::Ok()
}