use actix_web::{http, web, HttpMessage, HttpRequest, HttpResponse};
use service::sea_orm;
use entity::likes;
use service::{
    sea_orm::DatabaseConnection, LikesService, MutationRepository, UserService,
};

use crate::payload::ErrorResponse;

#[actix_web::post("/likes/{recipe_id}")]
async fn create_like(
    db: web::Data<DatabaseConnection>,
    recipe_id: web::Path<i32>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    let like = likes::ActiveModel {
        id: sea_orm::NotSet,
        user_id: sea_orm::Set(user_id),
        recipe_id: sea_orm::Set(recipe_id.into_inner()),
    };

    match LikesService::create(db.as_ref(), like).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error registering user's like",
            Some(err.to_string()),
        )),
    }
}

#[actix_web::get("/likes/")]
async fn fetch_user_likes(db: web::Data<DatabaseConnection>, req: HttpRequest) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    match UserService::find_user_recipes(db.as_ref(), user_id).await {
        Ok(vals) => {
            let likes: Vec<_> = vals.into_iter().filter_map(|(_, l)| l).collect();
            HttpResponse::Ok().json(likes)
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error retrieving user's likes",
            Some(err.to_string()),
        )),
    }
}

// #[actix_web::delete("/recipes/{like_id}")]
// async fn remove_like(db: web::Data<DatabaseConnection>, lke_id: web::Path<i32>) -> HttpResponse {
//
// }
