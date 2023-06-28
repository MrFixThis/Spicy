use actix_multipart::form::MultipartForm;
use actix_web::cookie::Cookie;
use actix_web::{http, web, HttpResponse};
use entity::{user, user_profile};
use service::sea_orm;
use service::{
    sea_orm::{ColumnTrait, DatabaseConnection, TryIntoModel},
    MutationRepository, QueryRepository, UserProfileService, UserService,
};

use crate::auth::password::verify_password;
use crate::auth::token::{issue_token, TokenKind};
use crate::error::{Error, Result};
use crate::payload::user_payload::*;

#[actix_web::post("/register/")]
async fn register_user(
    db: web::Data<DatabaseConnection>,
    new_user: MultipartForm<NewUser>,
) -> Result<HttpResponse> {
    check_email_existence(db.as_ref(), &new_user.email).await?;

    let new_user = new_user.into_inner().into_user_active_model().await;
    match UserService::create(db.as_ref(), new_user).await {
        Ok(nu) => {
            let nu_profile = user_profile::ActiveModel {
                id: sea_orm::NotSet,
                user_id: sea_orm::Set(nu.id),
                bio: sea_orm::Set(None),
                birth_date: sea_orm::Set(None),
                phone_number: sea_orm::Set(None),
            };
            _ = UserProfileService::create(db.as_ref(), nu_profile).await;

            Ok(HttpResponse::Created().json(nu.try_into_model().unwrap()))
        }
        Err(_) => Err(Error::InternalError("error registering user".to_string())),
    }
}

#[actix_web::post("/login/")]
async fn login(
    db: web::Data<DatabaseConnection>,
    cred: web::Json<UserCredetials>,
) -> Result<HttpResponse> {
    let UserCredetials { email, password } = cred.into_inner();
    match service::UserService::find_for_login(db.as_ref(), &email).await {
        Ok(user) => match user {
            Some(user) => match verify_password(password.as_bytes(), &user.password) {
                Ok(()) => {
                    let rt = issue_token(TokenKind::Refresh(user.id.to_string())).unwrap();
                    let at = issue_token(TokenKind::Access(user.id.to_string())).unwrap();
                    Ok(HttpResponse::Ok()
                        .cookie({
                            let mut cookie = Cookie::new("refresh_token", rt);
                            cookie.set_http_only(true);
                            cookie
                        })
                        .cookie(Cookie::new("access_token", at))
                        .json(Into::<LoggedInUser>::into(user)))
                }
                Err(_) => Err(Error::AuthError("invalid credentials".to_string())),
            },
            None => Err(Error::FetchError(
                http::StatusCode::NOT_FOUND,
                "user not registered or disabled".to_string(),
            )),
        },
        Err(_) => Err(Error::InternalError("error logging in user".to_string())),
    }
}

async fn check_email_existence(db: &DatabaseConnection, email: &str) -> Result<()> {
    match UserService::find_by(db, user::Column::Email.eq(email)).await {
        Ok(user) if user.is_some() => Err(Error::FetchError(
            http::StatusCode::BAD_REQUEST,
            "the given email is already in use".to_string(),
        )),
        Err(_) => Err(Error::InternalError(
            "error during user registration".to_string(),
        )),
        _ => Ok(()),
    }
}
