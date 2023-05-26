use entity::{prelude::RecipeImage, recipe_image};

use crate::{pk_ty, MutationRepository, QueryRepository};

#[derive(Debug)]
pub struct ImageRecipeService;
pk_ty!(recipe_image::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<RecipeImage, PrimaryKey> for ImageRecipeService {}

#[async_trait::async_trait]
impl MutationRepository<RecipeImage, recipe_image::ActiveModel, PrimaryKey> for ImageRecipeService {}
