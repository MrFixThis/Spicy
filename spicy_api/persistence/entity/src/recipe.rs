use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "recipe")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(default)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(Some(255))")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub preparation_steps: String,
    pub cooking_time: Time,
    #[sea_orm(default_value = true)]
    pub is_visible: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::likes::Entity")]
    Likes,
    #[sea_orm(has_many = "super::recipe_image::Entity")]
    RecipeImage,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::likes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl Related<super::recipe_image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeImage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
