mod auth;
mod middleware;
mod payload;
mod routes;
mod settings;
mod utils;

use actix_web::{dev, web::Data, App, HttpServer};
use service::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

const STATIC_DATA_PATH: &str = "static_data";

/// Setups the application's core configurations.
///
/// Several items get cofigured here, those like:
/// * [actix_web::dev::Server] cofigurations, and
/// * [service::sea_orm::DatabaseConnection] configurations.
pub async fn setup_app() -> anyhow::Result<dev::Server> {
    dotenvy::dotenv()?;
    let setts = settings::load_app_settings()?;

    let db = setup_conn().await?;
    entity::setup_db_schema(&db).await?;

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default() // HACK: Improve CORS constraints
            .allow_any_header()
            .allow_any_method()
            // .allowed_origin(&setts.frontend_url)
            .allow_any_origin() //NOTE: while testing
            .max_age(3000);

        App::new()
            .wrap(cors)
            .service(routes::sessions::login)
            .service(routes::sessions::register_user)
            .service(routes::health_check)
            .configure(routes::users_config)
            .app_data(Data::new(db.clone()))
            .service(actix_files::Files::new("/static/", STATIC_DATA_PATH).show_files_listing())
    })
    .bind(setts.server.socket_address())
    .map_err(anyhow::Error::msg)?
    .run();

    Ok(server)
}

/// Establishes the connection to the data source.
async fn setup_conn() -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL")?);
    opt.sqlx_logging(false);
    // .. further configs

    Database::connect(opt).await.map_err(anyhow::Error::msg)
}
