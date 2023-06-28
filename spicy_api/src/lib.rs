mod auth;
mod error;
mod middleware;
mod payload;
mod routes;
mod settings;
mod utils;

use actix_web::{dev, guard, web::Data, App, HttpServer};
use settings::AppSettings;

const STATIC_DATA_PATH: &str = "static_data";

/// Setups the application's core configurations.
///
/// Several items get cofigured here, those like:
/// * [actix_web::dev::Server] cofigurations, and
/// * [service::sea_orm::DatabaseConnection] configurations.
pub async fn setup_app() -> anyhow::Result<dev::Server> {
    dotenvy::dotenv()?;
    settings::parse_app_settings()?;

    let settings = AppSettings::get();
    let db = entity::setup_database(settings.datasource.clone().into_raw_format()).await?;

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default() // HACK: Improve CORS constraints
            .allow_any_header()
            .allow_any_method()
            .allowed_origin(&settings.frontend_url)
            .max_age(300);

        let static_content = actix_files::Files::new("/static/", STATIC_DATA_PATH)
            .guard(guard::Header("Host", &settings.frontend_url));

        App::new()
            .wrap(cors)
            .service(static_content)
            .service(routes::session::login)
            .service(routes::session::register_user)
            .service(routes::health_check::check_availability)
            .configure(routes::users_config)
            .app_data(Data::new(db.clone()))
    })
    .bind(settings.server.socket_address())
    .map_err(anyhow::Error::msg)?
    .run();

    Ok(server)
}
