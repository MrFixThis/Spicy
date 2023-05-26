mod persistence;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let _db = persistence::setup_conn().await?;

    Ok(())
}
