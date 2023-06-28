use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use entity::{user, user_profile};
use serde::{Deserialize, Serialize};
use service::sea_orm;
use service::sea_orm::prelude::DateTimeUtc;
use uuid::Uuid;

use crate::auth::password;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct UserCredetials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoggedInUser {
    pub name: String,
    pub surname: String,
    pub thumbnail: Option<String>,
    pub is_admin: bool,
}

impl From<user::Model> for LoggedInUser {
    fn from(value: user::Model) -> Self {
        Self {
            name: value.name,
            surname: value.surname,
            thumbnail: value.thumbnail,
            is_admin: value.is_admin,
        }
    }
}

#[derive(Debug, MultipartForm)]
pub struct NewUser {
    pub email: Text<String>,
    pub password: Text<String>,
    pub name: Text<String>,
    pub surname: Text<String>,
    #[multipart(limit = "32 MiB")]
    pub thumbnail: Option<TempFile>,
}

impl NewUser {
    pub async fn into_user_active_model(self) -> user::ActiveModel {
        let password = password::hash_password(self.password.into_inner().as_bytes()).unwrap();
        let thumbnail = match self.thumbnail {
            Some(tf) => utils::persist_file("users/", tf).await,
            None => None,
        };

        user::ActiveModel {
            id: sea_orm::Set(Uuid::new_v4()),
            email: sea_orm::Set(self.email.into_inner()),
            password: sea_orm::Set(password),
            name: sea_orm::Set(self.name.into_inner()),
            surname: sea_orm::Set(self.surname.into_inner()),
            is_admin: sea_orm::Set(false),
            is_active: sea_orm::Set(true),
            thumbnail: sea_orm::Set(thumbnail),
            created_at: sea_orm::Set(chrono::Utc::now()),
            updated_at: sea_orm::Set(None),
        }
    }
}

#[derive(Debug, MultipartForm)]
pub struct UpdatedUser {
    pub email: Option<Text<String>>,
    pub password: Option<Text<String>>,
    pub name: Option<Text<String>>,
    pub surname: Option<Text<String>>,
    pub phone_number: Option<Text<String>>,
    pub birth_date: Option<Text<chrono::NaiveDate>>,
    pub bio: Option<Text<String>>,
    #[multipart(limit = "32 MiB")]
    pub thumbnail: Option<TempFile>,
}

impl UpdatedUser {
    pub async fn into_active_model_pair(
        self,
        from: (user::Model, user_profile::Model),
    ) -> (user::ActiveModel, user_profile::ActiveModel) {
        let (st_user, st_user_prof) = from;

        #[rustfmt::skip]
        let user = {
            let password = match self.password {
                Some(psw) => {
                    sea_orm::Set(password::hash_password(psw.into_inner().as_bytes()).unwrap())
                }
                None => sea_orm::Unchanged(st_user.password),
            };
            let email = self.email.map_or_else(
                || sea_orm::Unchanged(st_user.email),
                |a| sea_orm::Set(a.into_inner())
            );
            let name = self.name.map_or_else(
                || sea_orm::Unchanged(st_user.name),
                |a| sea_orm::Set(a.into_inner())
            );
            let surname = self.surname.map_or_else(
                || sea_orm::Unchanged(st_user.surname),
                |a| sea_orm::Set(a.into_inner())
            );
            let thumbnail = match self.thumbnail {
                Some(tf) => match st_user.thumbnail {
                    Some(fl) => {
                        _ = tokio::fs::remove_file(fl).await;
                        sea_orm::Set(utils::persist_file("users/", tf).await)
                    }
                    None => sea_orm::Unchanged(None),
                },
                None => sea_orm::Set(None),
            };

            user::ActiveModel {
                id: sea_orm::Unchanged(st_user.id),
                email,
                password,
                name,
                surname,
                is_admin: sea_orm::Set(st_user.is_admin),
                is_active: sea_orm::Set(st_user.is_active),
                thumbnail,
                created_at: sea_orm::Unchanged(st_user.created_at),
                updated_at: sea_orm::Set(Some(chrono::Utc::now())),
            }
        };

        #[rustfmt::skip]
        let user_profile = {
            let birth_date = self.birth_date.map_or_else(
                || sea_orm::Unchanged(st_user_prof.birth_date),
                |a| sea_orm::Set(Some(a.into_inner())),
            );
            let phone_number = self.phone_number.map_or_else(
                || sea_orm::Unchanged(st_user_prof.phone_number),
                |a| sea_orm::Set(Some(a.into_inner())),
            );
            let bio = self.bio.map_or_else(
                || sea_orm::Unchanged(st_user_prof.bio),
                |a| sea_orm::Set(Some(a.into_inner()))
            );

            user_profile::ActiveModel {
                id: sea_orm::Set(st_user_prof.id),
                user_id: sea_orm::Set(st_user.id),
                birth_date,
                phone_number,
                bio,
            }
        };

        (user, user_profile)
    }
}

#[derive(Debug, Serialize)]
pub struct WholeUserProfile {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub bio: Option<String>,
    pub thumbnail: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

impl From<(user::Model, user_profile::Model)> for WholeUserProfile {
    fn from(value: (user::Model, user_profile::Model)) -> Self {
        let (
            user::Model {
                email,
                name,
                surname,
                thumbnail,
                created_at,
                updated_at,
                ..
            },
            user_profile::Model {
                phone_number,
                birth_date,
                bio,
                ..
            },
        ) = value;
        Self {
            name,
            surname,
            email,
            phone_number,
            birth_date,
            bio,
            thumbnail,
            created_at,
            updated_at,
        }
    }
}
