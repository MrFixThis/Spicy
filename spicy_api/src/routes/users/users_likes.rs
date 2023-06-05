use actix_web::{http, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{likes, user};
use service::{sea_orm::DatabaseConnection, RelationService};

use crate::payload::ErrorResponse;

#[actix_web::get("/likes/")]
async fn fetch_user_likes(db: web::Data<DatabaseConnection>, req: HttpRequest) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    let likes =
        RelationService::load_many_by_pk::<user::Entity, _, _>(db.as_ref(), likes::Entity, user_id)
            .await;

    match likes {
        Ok(pair) => {
            match pair {
                Some((_, col)) => HttpResponse::Ok().json(col),
                None => HttpResponse::NotFound().json(ErrorResponse::new(
                    http::StatusCode::NOT_FOUND,
                    "the user does not have liked content",
                    None,
                )),
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error retrieving user's likes",
            Some(err.to_string()),
        )),
    }
}
