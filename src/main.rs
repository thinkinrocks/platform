mod bot;
mod models;
mod repository;
mod templates;
mod web;

use sqlx::sqlite::SqlitePoolOptions;
use tokio::task::JoinSet;

use crate::{bot::serve_bot, repository::Repository, web::serve_web};

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

    let mut join = JoinSet::new();

    join.spawn(serve_bot(repo.clone()));
    join.spawn(serve_web("0.0.0.0:3000", repo));

    join.join_all().await;

    Ok(())
}
