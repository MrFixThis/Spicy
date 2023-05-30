use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Copy, Default, DeriveEntity)]
#[sea_orm(table_name = "user")]
pub struct Entity;

#[derive(Debug, Clone, Copy, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Email,
    Password,
    Name,
    Surname,
    DateJoined,
    IsActive,
    Thumbnail,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Column::Id => ColumnType::Integer.def(),
            Column::Email => ColumnType::String(Some(255)).def().indexed().unique(),
            Column::Password => ColumnType::Text.def().indexed(),
            Column::Name => ColumnType::String(Some(60)).def(),
            Column::Surname => ColumnType::String(Some(60)).def(),
            Column::DateJoined => ColumnType::Date.def(),
            Column::IsActive => ColumnType::Boolean.def().indexed().default_value(true),
            Column::Thumbnail => ColumnType::Text.def().nullable(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub surname: String,
    pub date_joined: Date,
    pub is_active: bool,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user_profile::Entity")]
    UserProfile,
    #[sea_orm(has_many = "super::recipe::Entity")]
    Recipe,
    #[sea_orm(has_many = "super::likes::Entity")]
    Likes,
    #[sea_orm(has_many = "super::auditing::Entity")]
    Auditing,
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

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::Role.def().rev())
    }
}

impl Related<super::likes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Likes.def()
    }
}

impl Related<super::auditing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auditing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
