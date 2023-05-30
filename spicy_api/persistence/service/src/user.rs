use crate::{pk_ty, MutationRepository, QueryRepository};
use ::entity::{prelude::User, user};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QueryOrder, Condition};

#[derive(Debug)]
pub struct UserService;
pk_ty!(user::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<User, PrimaryKey> for UserService {
    async fn find_all(db: &DbConn) -> anyhow::Result<Vec<user::Model>> {
        User::find()
            .order_by_asc(user::Column::DateJoined)
            .all(db)
            .await
            .map_err(anyhow::Error::msg)
    }
}

impl UserService {
    pub async fn find_for_login(db: &DbConn, email: &str) -> anyhow::Result<Option<user::Model>> {
        User::find()
            .filter(
                Condition::all()
                    .add(user::Column::Email.eq(email))
                    .add(user::Column::IsActive.eq(true))
            )
            .one(db)
            .await
            .map_err(anyhow::Error::msg)
    }

    pub async fn find_inactive(db: &DbConn) -> anyhow::Result<Vec<user::Model>> {
        User::find()
            .filter(user::Column::IsActive.eq(false))
            .order_by_asc(user::Column::DateJoined)
            .all(db)
            .await
            .map_err(anyhow::Error::msg)
    }
}

#[async_trait::async_trait]
impl MutationRepository<User, user::ActiveModel, PrimaryKey> for UserService {}
