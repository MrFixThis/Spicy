use spicy_src;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    spicy_src::setup_app().await?.await?;
    Ok(())
}
