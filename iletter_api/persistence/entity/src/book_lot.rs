use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book_lot")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    book_isbn: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    price_per_unit: Decimal,
    available_units: i16,
    inventory_id: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
