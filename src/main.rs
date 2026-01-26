mod bot;
mod models;
mod repository;
mod templates;

use sqlx::sqlite::SqlitePoolOptions;
use tokio::task::JoinSet;

use crate::{bot::serve_bot, repository::Repository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    let repo = Repository::new(pool);

    let _ = [serve_bot(repo)]
        .into_iter()
        .collect::<JoinSet<_>>()
        .join_all()
        .await;

    Ok(())
}
