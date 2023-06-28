use crate::{pk_ty, MutationRepository, QueryRepository};
use ::entity::{prelude::User, recipe, user};
use entity::likes;
use sea_orm::{ColumnTrait, Condition, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder};

#[derive(Debug)]
pub struct UserService;
pk_ty!(user::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<User, PrimaryKey> for UserService {
    async fn find_by_pk(db: &DbConn, pk: PrimaryKey) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(pk)
            .one(db)
            .await
    }

    async fn find_all(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
        User::find()
            .order_by_asc(user::Column::CreatedAt)
            .all(db)
            .await
    }
}

#[async_trait::async_trait]
impl MutationRepository<User, user::ActiveModel, PrimaryKey> for UserService {}

impl UserService {
    pub async fn find_for_login(db: &DbConn, email: &str) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(
                Condition::all()
                    .add(user::Column::Email.eq(email))
                    .add(user::Column::IsActive.eq(true)),
            )
            .one(db)
            .await
    }

    pub async fn find_user_recipes(
        db: &DbConn,
        pk: PrimaryKey,
    ) -> Result<Vec<(user::Model, Option<recipe::Model>)>, DbErr> {
        User::find_by_id(pk)
            .find_also_linked(likes::Entity)
            .all(db)
            .await
    }
}
