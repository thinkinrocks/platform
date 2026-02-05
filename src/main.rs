mod config;
mod bot;
mod auth;
mod repository;

use log::{info};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::task::JoinSet;

use crate::{auth::Auth, bot::serve_bot, config::Config, repository::Repository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let config = envy::from_env::<Config>()?;

    let auth = Auth::new(&config.ldap_server, &config.ldap_username, &config.ldap_password, &config.ldap_base_dn, &config.ldap_user_filter).await;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await?;
    
    let repo = Repository::new(pool);

    info!("Using config: {:?}", config);

    let mut join = JoinSet::new();

    join.spawn(serve_bot(repo.clone()));

    info!("(half life scientist) everything.. seems to be in order");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            log::info!("Ctrl+C received, shutting down...");
        }

        res = join.join_next() => {
            if let Some(res) = res {
                if let Err(err) = res {
                    log::error!("Task failed: {:?}", err);
                }
            }
        }
    }

    join.abort_all();

    Ok(())
}
