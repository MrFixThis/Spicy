use entity::{prelude::Recipe, recipe};
use sea_orm::{DbConn, EntityTrait, QueryOrder};

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

impl RecipeService {}
