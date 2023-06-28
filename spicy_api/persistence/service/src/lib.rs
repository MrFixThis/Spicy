mod likes;
mod recipe;
mod recipe_image;
mod user;
mod user_profile;

pub use likes::LikesService;
pub use recipe::RecipeService;
pub use recipe_image::ImageRecipeService;
pub use sea_orm;
use sea_orm::{sea_query::IntoCondition, *};
pub use user::UserService;
pub use user_profile::UserProfileService;

#[macro_export]
macro_rules! pk_ty {
    ($pk:ty) => {
        pub type PrimaryKey = <$pk as ::sea_orm::PrimaryKeyTrait>::ValueType;
    };
}

#[async_trait::async_trait]
pub trait QueryRepository<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    async fn find_by_pk(db: &DbConn, pk: Pk) -> Result<Option<E::Model>, DbErr> {
        E::find_by_id(pk).one(db).await
    }

    async fn find_by<F>(db: &DbConn, expr: F) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::find().filter(expr).one(db).await
    }

    async fn find_all(db: &DbConn) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(db).await
    }

    async fn find_all_by<F>(db: &DbConn, expr: F) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::find().filter(expr).all(db).await
    }
}

#[async_trait::async_trait]
pub trait MutationRepository<E, A, Pk>: QueryRepository<E, Pk>
where
    E: EntityTrait,
    E::Model: IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior<Entity = E> + Send + 'static,
    A: TryIntoModel<E::Model>,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    async fn create(db: &DbConn, act_mod: A) -> Result<E::Model, DbErr> {
        act_mod.insert(db).await
    }

    async fn update(db: &DbConn, act_mod: A) -> Result<E::Model, DbErr> {
        act_mod.save(db).await.and_then(|m| m.try_into_model())
    }

    async fn delete_by_pk(db: &DbConn, pk: Pk) -> Result<DeleteResult, DbErr> {
        E::delete_by_id(pk).exec(db).await
    }
    async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        E::delete_many().exec(db).await
    }

    async fn delete_all_by<F>(db: &DbConn, expr: F) -> Result<DeleteResult, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::delete_many().filter(expr).exec(db).await
    }
}

#[derive(Debug)]
pub struct RelationService;

impl RelationService {
    pub async fn load_one_by_pk<E, Pk, F>(
        db: &DbConn,
        target: F,
        pk: Pk,
    ) -> Result<Option<(E::Model, Option<F::Model>)>, DbErr>
    where
        F: EntityTrait,
        F::Model: Send + Sync,
        E: EntityTrait + Related<F>,
        E::Model: Send + Sync,
        Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
    {
        E::find_by_id(pk).find_also_related(target).one(db).await
    }

    pub async fn load_many_by_pk<E, Pk, F>(
        db: &DbConn,
        target: F,
        pk: Pk,
    ) -> Result<Option<(E::Model, Vec<F::Model>)>, DbErr>
    where
        F: EntityTrait,
        F::Model: Send + Sync,
        E: EntityTrait + Related<F>,
        E::Model: Send + Sync,
        Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
    {
        E::find_by_id(pk)
            .find_with_related(target)
            .all(db)
            .await
            .map(|mut e| e.pop())
    }

    pub async fn load_many_to_many<E, V, F, S>(
        db: &DbConn,
        other: S,
        via: V,
    ) -> Result<Vec<Vec<F::Model>>, DbErr>
    where
        F: EntityTrait,
        F::Model: Send + Sync,
        E: EntityTrait + Related<F>,
        E::Model: Send + Sync,
        V: EntityTrait,
        V::Model: Send + Sync,
        S: EntityOrSelect<F>,
    {
        E::find()
            .all(db)
            .await?
            .load_many_to_many(other, via, db)
            .await
    }
}
