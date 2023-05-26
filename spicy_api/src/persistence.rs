use anyhow::anyhow;
use service::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};

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

    Database::connect(opt).await.map_err(|e| anyhow!(e))
}
