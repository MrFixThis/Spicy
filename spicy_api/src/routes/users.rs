pub mod users_likes;
pub mod users_recipes;

use actix_multipart::form::MultipartForm;
use actix_web::{http, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{user, user_profile};
use service::{
    sea_orm::{
        ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, TransactionTrait, TryIntoModel,
    },
    MutationRepository, QueryRepository, RelationService, UserProfileService, UserService,
};

use crate::payload::{user_payload::*, ErrorResponse};

#[actix_web::post("/register/")]
async fn register_user(
    db: web::Data<DatabaseConnection>,
    new_user: MultipartForm<NewUser>,
) -> HttpResponse {
    match UserService::find_by(db.as_ref(), user::Column::Email.eq(&new_user.email.clone())).await {
        Ok(user) if user.is_some() => {
            return HttpResponse::Conflict().json(ErrorResponse::new(
                http::StatusCode::CONFLICT,
                "the given email is already in use",
                None,
            ))
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResponse::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "the given email is already in use",
                Some(err.to_string()),
            ))
        }
        _ => (),
    }

    let new_user = new_user.into_inner().into_user_model().await;
    match UserService::create(db.as_ref(), new_user).await {
        Ok(nu) => {
            let nu_profile = user_profile::Model {
                id: 0,
                user_id: nu.id.as_ref().to_owned(),
                bio: None,
                birth_date: None,
                phone_number: None,
            };
            _ = UserProfileService::create(db.as_ref(), nu_profile).await;

            HttpResponse::Created()
                .append_header(("Location", "/login"))
                .json(nu.try_into_model().unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error registering user",
            Some(err.to_string()),
        )),
    }
}

#[actix_web::get("/")]
async fn fetch_user(db: web::Data<DatabaseConnection>, req: HttpRequest) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    match service::UserService::find_by_pk(db.as_ref(), user_id).await {
        Ok(res) => {
            match res {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().json(ErrorResponse::new(
                    http::StatusCode::NOT_FOUND,
                    "user not found",
                    None
                )),
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error fetching user's information",
            Some(err.to_string())
        )),
    }
}

#[actix_web::get("/all/")]
async fn fetch_all_users(db: web::Data<DatabaseConnection>, _: HttpRequest) -> HttpResponse {
    match service::UserService::find_all(db.as_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error fetching users' information",
            Some(err.to_string())
        )),
    }
}

#[actix_web::put("/update/")]
async fn update_user(
    db: web::Data<DatabaseConnection>,
    updated_user: MultipartForm<UpdatedUser>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    let user_and_prof = RelationService::load_one_by_pk::<user::Entity, _, _>(
        db.as_ref(),
        user_profile::Entity,
        user_id,
    )
    .await
    .map(Option::unwrap)
    .map(|(u, p)| (u, p.unwrap()))
    .unwrap();

    let tnx_res = db
        .transaction::<_, (user::Model, user_profile::Model), DbErr>(|tnx| {
            Box::pin(async move {
                let (user, profile) = updated_user
                    .into_inner()
                    .into_active_model_pair(user_and_prof)
                    .await;

                Ok((
                    user.save(tnx).await?.try_into_model().unwrap(),
                    profile.save(tnx).await?.try_into_model().unwrap(),
                ))
            })
        })
        .await;

    match tnx_res {
        Ok((u, _)) => HttpResponse::Ok().json(u),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error updating user",
            Some(err.to_string()),
        )),
    }
}

#[actix_web::post("/delete/")]
async fn delete_user(db: web::Data<DatabaseConnection>, req: HttpRequest) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    match UserService::delete_by_pk(db.as_ref(), user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error deleting user",
            Some(err.to_string()),
        )),
    }
}

// TODO: User enabling/disaling handlers
