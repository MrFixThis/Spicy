use entity::{prelude::Recipe, recipe};
use sea_orm::{ColumnTrait, DbConn, DeleteResult, EntityTrait, QueryFilter, QueryOrder};

use crate::{pk_ty, MutationRepository, QueryRepository};

#[derive(Debug)]
pub struct RecipeService;
pk_ty!(recipe::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Recipe, PrimaryKey> for RecipeService {
    async fn find_all(db: &DbConn) -> anyhow::Result<Vec<recipe::Model>> {
        Recipe::find()
            .order_by_asc(recipe::Column::DateCreated)
            .all(db)
            .await
            .map_err(anyhow::Error::msg)
    }
}

#[async_trait::async_trait]
impl MutationRepository<Recipe, recipe::ActiveModel, PrimaryKey> for RecipeService {}

impl RecipeService {
    pub async fn find_all_not_visible(db: &DbConn) -> anyhow::Result<Vec<recipe::Model>> {
        Recipe::find()
            .filter(recipe::Column::IsVisible.eq(false))
            .order_by_asc(recipe::Column::DateCreated)
            .all(db)
            .await
            .map_err(anyhow::Error::msg)
    }

    pub async fn delete_all_not_visible(db: &DbConn) -> anyhow::Result<DeleteResult> {
        Recipe::delete_many()
            .filter(recipe::Column::IsVisible.eq(false))
            .exec(db)
            .await
            .map_err(anyhow::Error::msg)
    }
}
