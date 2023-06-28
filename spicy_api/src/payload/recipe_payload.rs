use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use entity::recipe;
use service::sea_orm;
use uuid::Uuid;

use crate::utils::persist_file;

#[derive(Debug, MultipartForm)]
pub struct NewRecipe {
    pub title: Text<String>,
    pub description: Text<String>,
    pub preparation_steps: Text<String>,
    pub cooking_time: Text<String>,
    pub recipe_images: Vec<TempFile>,
}

impl NewRecipe {
    pub async fn into_user_recipe(
        self,
        user_id: &Uuid,
    ) -> (recipe::ActiveModel, Option<Vec<String>>) {
        let images = if !self.recipe_images.is_empty() {
            let mut paths: Vec<String> = Vec::with_capacity(self.recipe_images.len());
            for img in self.recipe_images {
                if let Some(path) = persist_file("recipes/", img).await {
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
                id: sea_orm::Set(Uuid::new_v4()),
                user_id: sea_orm::Set(user_id.to_owned()),
                title: sea_orm::Set(self.title.into_inner()),
                description: sea_orm::Set(self.description.into_inner()),
                preparation_steps: sea_orm::Set(self.preparation_steps.into_inner()),
                cooking_time,
                is_visible: sea_orm::Set(true),
                created_at: sea_orm::Set(chrono::Utc::now()),
                updated_at: sea_orm::Set(None),
            },
            images,
        )
    }
}
