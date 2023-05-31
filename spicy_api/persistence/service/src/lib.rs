mod auditing;
mod likes;
mod recipe;
mod recipe_image;
mod role;
mod user;
mod user_profile;

pub use auditing::AuditingService;
pub use likes::LikesService;
pub use recipe::RecipeService;
pub use recipe_image::ImageRecipeService;
pub use role::RoleService;
pub use sea_orm;
use sea_orm::*;
pub use user::UserService;
pub use user_profile::UserProfileService;

#[async_trait::async_trait]
pub trait QueryRepository<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    async fn find_by_pk(db: &DbConn, pk: Pk) -> anyhow::Result<Option<E::Model>> {
        E::find_by_id(pk).one(db).await.map_err(anyhow::Error::msg)
    }

    async fn find_all(db: &DbConn) -> anyhow::Result<Vec<E::Model>> {
        E::find().all(db).await.map_err(anyhow::Error::msg)
    }
}

#[async_trait::async_trait]
pub trait MutationRepository<E, A, Pk>: QueryRepository<E, Pk>
where
    E: EntityTrait,
    E::Model: IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior<Entity = E> + Send,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    async fn create(db: &DbConn, from_mod: E::Model) -> anyhow::Result<A> {
        from_mod
            .into_active_model()
            .save(db)
            .await
            .map_err(anyhow::Error::msg)
    }

    async fn update_by<V>(
        db: &DbConn,
        col: E::Column,
        col_val: V,
        from_mod: E::Model,
    ) -> anyhow::Result<A>
    where
        V: Into<Value> + Send
    {
        let mut ent = from_mod.into_active_model();

        ent.set(col, col_val.into());
        ent.reset_all().save(db).await.map_err(anyhow::Error::msg)
    }

    async fn delete_by_pk(db: &DbConn, pk: Pk) -> anyhow::Result<DeleteResult> {
        E::delete_by_id(pk)
            .exec(db)
            .await
            .map_err(anyhow::Error::msg)
    }

    async fn delete_all(db: &DbConn) -> anyhow::Result<DeleteResult> {
        E::delete_many().exec(db).await.map_err(anyhow::Error::msg)
    }
}

#[derive(Debug)]
pub struct RelationService;

impl RelationService {
    pub async fn load_one<E, F, S>(db: &DbConn, target: S) -> anyhow::Result<Vec<Option<F::Model>>>
    where
        F: EntityTrait,
        F::Model: Send + Sync,
        E: EntityTrait + Related<F>,
        E::Model: Send + Sync,
        S: EntityOrSelect<F>,
    {
        E::find()
            .all(db)
            .await?
            .load_one(target, db)
            .await
            .map_err(anyhow::Error::msg)
    }

    pub async fn load_many<E, F, S>(db: &DbConn, target: S) -> anyhow::Result<Vec<Vec<F::Model>>>
    where
        F: EntityTrait,
        F::Model: Send + Sync,
        E: EntityTrait + Related<F>,
        E::Model: Send + Sync,
        S: EntityOrSelect<F>,
    {
        E::find()
            .all(db)
            .await?
            .load_many(target, db)
            .await
            .map_err(anyhow::Error::msg)
    }

    pub async fn load_many_to_many<E, F, V, S>(
        db: &DbConn,
        other: S,
        via: V,
    ) -> anyhow::Result<Vec<Vec<F::Model>>>
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
            .map_err(anyhow::Error::msg)
    }
}

#[macro_export]
macro_rules! pk_ty {
    ($pk:ty) => {
        pub type PrimaryKey = <$pk as ::sea_orm::PrimaryKeyTrait>::ValueType;
    };
}
