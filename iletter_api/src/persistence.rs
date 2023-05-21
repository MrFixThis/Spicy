use std::{env, time::Duration};
use anyhow::anyhow;
use entity::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

/// Establishes the connection to the data source and runs the initial
/// migration on startup.
pub async fn setup_conn() -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL")?);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(5))
        .sqlx_logging(false);

    let db = Database::connect(opt).await.map_err(|e| anyhow!(e))?;
    Migrator::up(&db, None).await.map_err(|e| anyhow!(e))?;
    Ok(db)
}
