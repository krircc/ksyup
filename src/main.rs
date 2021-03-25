use ksyup::config::Config;
use ksyup::run;
use color_eyre::Result;
use sqlx::PgPool;

#[ntex::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env()?;
    let db_url = &settings.database_url;

    // Installation de Color Eyre
    // --------------------------
    color_eyre::install()?;

    // Initialisation du pool Postgres
    // ----------------------------
    let db_pool = PgPool::connect(db_url).await?;

    run(settings, db_pool).await
}
