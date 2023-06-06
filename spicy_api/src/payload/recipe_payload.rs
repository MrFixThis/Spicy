use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use entity::recipe;
use service::sea_orm;

use crate::utils::try_persist_file;

#[derive(Debug, MultipartForm)]
pub struct NewRecipe {
    title: Text<String>,
    description: Text<String>,
    preparation_steps: Text<String>,
    cooking_time: Text<String>,
    recipe_images: Vec<TempFile>,
}

impl NewRecipe {
    pub async fn into_user_recipe(
        self,
        user_id: i32,
    ) -> (recipe::ActiveModel, Option<Vec<String>>) {
        let images = if !self.recipe_images.is_empty() {
            let mut paths: Vec<String> = Vec::with_capacity(self.recipe_images.len());
            for img in self.recipe_images {
                if let Some(path) = try_persist_file("recipes/", img).await {
                    paths.push(path)
                }
            }
            Some(paths)
        } else {
            None
        };

        let cooking_time = sea_orm::Set(
            chrono::NaiveTime::parse_from_str(&self.cooking_time.into_inner(), "%H:%M:%S")
                .unwrap_or_else(|_| chrono::NaiveTime::default()),
        );

        (
            recipe::ActiveModel {
                id: sea_orm::NotSet,
                title: sea_orm::Set(self.title.into_inner()),
                date_created: sea_orm::Set(chrono::Local::now().date_naive()),
                description: sea_orm::Set(self.description.into_inner()),
                preparation_steps: sea_orm::Set(self.preparation_steps.into_inner()),
                cooking_time,
                is_visible: sea_orm::Set(true), // default
                user_id: sea_orm::Set(user_id),
            },
            images,
        )
    }
}
