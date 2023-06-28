use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, DeriveEntity)]
#[sea_orm(table_name = "user")]
pub struct Entity;

#[derive(Debug, Clone, Copy, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Email,
    Password,
    Name,
    Surname,
    IsAdmin,
    IsActive,
    Thumbnail,
    CreatedAt,
    UpdatedAt,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Column::Id => ColumnType::Uuid.def(),
            Column::Email => ColumnType::String(Some(255)).def().indexed().unique(),
            Column::Password => ColumnType::Text.def().indexed(),
            Column::Name => ColumnType::String(Some(60)).def(),
            Column::Surname => ColumnType::String(Some(60)).def(),
            Column::IsAdmin => ColumnType::Boolean.def(),
            Column::IsActive => ColumnType::Boolean.def().indexed().default_value(true),
            Column::Thumbnail => ColumnType::Text.def().nullable(),
            Column::CreatedAt => ColumnType::Timestamp.def(),
            Column::UpdatedAt => ColumnType::Timestamp.def().nullable(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub surname: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub thumbnail: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user_profile::Entity")]
    UserProfile,
    #[sea_orm(has_many = "super::recipe::Entity")]
    Recipe,
    #[sea_orm(has_many = "super::likes::Entity")]
    Likes,
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipe.def()
    }
}

impl Related<super::likes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
