pub mod users_likes;
pub mod users_recipes;

use actix_multipart::form::MultipartForm;
use actix_web::{http, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{user, user_profile};
use service::{
    sea_orm::{
        ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel, TransactionTrait,
        TryIntoModel,
    },
    MutationRepository, QueryRepository, RelationService, UserService,
};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::payload::user_payload::*;

#[actix_web::get("/")]
async fn fetch_user(db: web::Data<DatabaseConnection>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().unwrap().to_owned();
    match service::UserService::find_by_pk(db.as_ref(), user_id).await {
        Ok(res) => match res {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Err(Error::FetchError(
                http::StatusCode::NOT_FOUND,
                format!(r#"user with id "{user_id}" not found"#),
            )),
        },
        Err(err) => Err(Error::InternalError(format!(
            r#"error fetching user's information ("{}")"#,
            err.to_string()
        ))),
    }
}

#[actix_web::get("/all/")]
async fn fetch_all_users(db: web::Data<DatabaseConnection>) -> Result<HttpResponse> {
    match service::UserService::find_all(db.as_ref()).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => Err(Error::InternalError(format!(
            r#"error fetching user's information ("{}")"#,
            err.to_string()
        ))),
    }
}

#[actix_web::put("/update/")]
async fn update_user(
    db: web::Data<DatabaseConnection>,
    updated_user: MultipartForm<UpdatedUser>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().unwrap().to_owned();
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
        Ok(pair) => Ok(HttpResponse::Ok().json(Into::<WholeUserProfile>::into(pair))),
        Err(_) => Err(Error::InternalError("error updating user".to_string())),
    }
}

#[actix_web::patch("/toggle_state/")]
async fn toggle_user_state(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().unwrap().to_owned();
    let db = db.as_ref();

    match UserService::find_by_pk(db, user_id).await {
        Ok(res) => match res {
            Some(usr) => {
                let mut disabled_user: user::ActiveModel = usr.into_active_model();
                let state: bool = disabled_user.get(user::Column::IsActive).unwrap().unwrap();
                disabled_user.set(user::Column::IsActive, (!state).into());
                Ok(HttpResponse::Ok().json(UserService::update(db, disabled_user).await.unwrap()))
            }
            None => Err(Error::FetchError(
                http::StatusCode::NOT_FOUND,
                format!(r#"user with id "{user_id}" not found"#),
            )),
        },
        Err(err) => Err(Error::InternalError(format!(
            r#"error fetching user's information ("{}")"#,
            err.to_string()
        ))),
    }
}
