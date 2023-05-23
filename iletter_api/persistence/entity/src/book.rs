use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub isbn: Uuid,
    pub name: String,
    pub author_name: String,
    pub publisher_name: String,
    pub number_of_pages: u16,
    pub publication_date: Date,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
