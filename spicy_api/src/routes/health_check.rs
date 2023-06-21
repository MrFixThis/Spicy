use actix_web::{HttpResponse, Responder};
use serde_json::json;

#[actix_web::get("/health_check")]
pub async fn check_availability() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "msg": "Spicy's API Running"
    }))
}
