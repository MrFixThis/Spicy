use sea_orm::{ConnectOptions, ConnectionTrait, Database, DbConn, DbErr, Statement};

pub mod likes;
pub mod prelude;
pub mod recipe;
pub mod recipe_image;
pub mod user;
pub mod user_profile;

macro_rules! tables {
    {
        connection=$db:expr,
        entities=[ $( $ety:expr ),+ ]
    } => {
        {
            let backend = $db.get_database_backend();
            let sch = ::sea_orm::Schema::new(backend);
            let tables = ::std::vec::Vec::from(
                [ $( backend.build(sch.create_table_from_entity($ety).if_not_exists()) ),+ ]
            );

            for tb in tables {
                $db.execute(tb).await?;
            }
        }
    };
}

pub async fn setup_database(db_cred: (String, String)) -> Result<DbConn, DbErr> {
    let db = Database::connect(&db_cred.0).await?;
    let res = db
        .execute(Statement::from_string(
            db.get_database_backend(),
            format!(r"CREATE DATABASE IF NOT EXISTS {};", &db_cred.1),
        ))
        .await?;

    let mut opts = ConnectOptions::new(format!("{}/{}", db_cred.0, db_cred.1));
    opts.sqlx_logging(false);
    // further configs ...
    let conn = Database::connect(opts).await?;
    if res.rows_affected() > 0 {
        tables! {
            connection = conn,
            entities = [
                user::Entity,
                user_profile::Entity,
                recipe::Entity,
                recipe_image::Entity,
                likes::Entity
            ]
        }
    }
    Ok(conn)
}
