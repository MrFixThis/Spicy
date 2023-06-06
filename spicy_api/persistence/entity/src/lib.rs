use sea_orm::{ConnectionTrait, DbConn, DbErr, Schema};

pub mod prelude;

pub mod auditing;
pub mod likes;
pub mod recipe;
pub mod recipe_image;
pub mod role;
pub mod user;
pub mod user_profile;
pub mod user_role;

// Generates the setups the database's schema based on the entities defined earlier.
pub async fn setup_db_schema(db: &DbConn) -> Result<(), DbErr> {
    let backend = db.get_database_backend();
    let sch = Schema::new(backend);
    let statements = [
        backend.build(sch.create_table_from_entity(user::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(user_profile::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(role::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(user_role::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(recipe::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(recipe_image::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(likes::Entity).if_not_exists()),
        backend.build(sch.create_table_from_entity(auditing::Entity).if_not_exists()),
    ];

    for stm in statements {

        db.execute(stm).await?;
    }

    Ok(())
}
