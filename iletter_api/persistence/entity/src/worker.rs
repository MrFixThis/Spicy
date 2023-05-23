use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "worker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub first_last_name: String,
    pub second_last_name: Option<String>,
    pub identification: String,
    pub date_of_birth: Date,
    pub email_address: String,
    pub phone_number: String,
    pub residence_address: String,
    pub date_of_hire: Date,
    pub role: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub basic_salary: Decimal,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
