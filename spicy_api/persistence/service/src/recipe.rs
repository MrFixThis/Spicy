use entity::{prelude::Recipe, recipe};
use sea_orm::{DbConn, DbErr, EntityTrait, QueryOrder};

use crate::{pk_ty, MutationRepository, QueryRepository};

#[derive(Debug)]
pub struct RecipeService;
pk_ty!(recipe::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Recipe, PrimaryKey> for RecipeService {
    async fn find_all(db: &DbConn) -> Result<Vec<recipe::Model>, DbErr> {
        Recipe::find()
            .order_by_asc(recipe::Column::CreatedAt)
            .all(db)
            .await
    }
}

#[async_trait::async_trait]
impl MutationRepository<Recipe, recipe::ActiveModel, PrimaryKey> for RecipeService {}

impl RecipeService {}
