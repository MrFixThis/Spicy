use actix_multipart::form::MultipartForm;
use actix_web::{http, web, HttpResponse};
use entity::{user, user_profile};
use service::sea_orm;
use service::{
    sea_orm::{ColumnTrait, DatabaseConnection, TryIntoModel},
    MutationRepository, QueryRepository, UserProfileService, UserService,
};

use crate::auth::password::verify_password;
use crate::auth::token::issue_token;
use crate::payload::{user_payload::*, ErrorResponse};

#[actix_web::post("/register/")]
async fn register_user(
    db: web::Data<DatabaseConnection>,
    new_user: MultipartForm<NewUser>,
) -> HttpResponse {
    match check_email_existence(db.as_ref(), new_user.email.clone()).await {
        Err(res) => return res,
        _ => (),
    }

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

            HttpResponse::Created().json(nu.try_into_model().unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error registering user",
            Some(err.to_string()),
        )),
    }
}

#[actix_web::post("/login/")]
async fn login(db: web::Data<DatabaseConnection>, cred: web::Json<UserCredetials>) -> HttpResponse {
    let UserCredetials { email, password } = cred.into_inner();
    match service::UserService::find_for_login(db.as_ref(), &email).await {
        Ok(user) => match user {
            Some(user) => {
                match verify_password(password.as_bytes(), &user.password) {
                    Ok(()) => {
                        let token = issue_token(user.id).unwrap();
                        HttpResponse::Ok().json(LoggedInUser::new(
                            user.name,
                            user.surname,
                            user.email,
                            vec![], // TODO: SEND THE ROLES!
                            token,
                        ))
                    }
                    Err(_) => HttpResponse::Unauthorized().json(ErrorResponse::new(
                        http::StatusCode::UNAUTHORIZED,
                        "invalid credentials",
                        None,
                    )),
                }
            }
            None => HttpResponse::NotFound().json(ErrorResponse::new(
                http::StatusCode::NOT_FOUND,
                "user not registered or disabled",
                None,
            )),
        },
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error logging in user",
            Some(err.to_string()),
        )),
    }
}

async fn check_email_existence(db: &DatabaseConnection, email: String) -> Result<(), HttpResponse> {
    match UserService::find_by(db, user::Column::Email.eq(email)).await {
        Ok(user) if user.is_some() => Err(HttpResponse::BadRequest().json(ErrorResponse::new(
            http::StatusCode::BAD_REQUEST,
            "the given email is already in use",
            None,
        ))),
        Err(err) => Err(HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error during user registration",
            Some(err.to_string()),
        ))),
        _ => Ok(()),
    }
}
