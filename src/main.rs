mod models;
mod repository;

use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use teloxide::{macros::BotCommands, prelude::*};

use crate::repository::Repository;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Me,
    Help,
    Start,
    Introduce(String),
    Search(String),
}

async fn handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    repo: Arc<Repository>,
) -> ResponseResult<()> {
    match cmd {
        Command::Introduce(_) => {
            bot.send_message(msg.chat.id, "Sorry! This command doesn't work yet. If you need it to work message @ktnlvr, he made the bot.").await?;
        }
        Command::Start | Command::Help => {
            bot.send_message(msg.chat.id, "Hello, this is the Thinkin' Rocks* bot")
                .await?;
        }
        Command::Me => {
            let username = msg.chat.username().unwrap();
            let user = repo.get_user(username).await.unwrap().unwrap();

            bot.send_message(
                msg.chat.id,
                format!(
                    "Hello, @{}!\nYour sire is {}",
                    user.telegram_username,
                    user.sire.unwrap_or("no one!".to_string())
                ),
            )
            .await
            .unwrap();
        }
        Command::Search(query) => {
            let entries = repo.search_entries(query, 15).await.unwrap();
            bot.send_message(msg.chat.id, format!("{:?}", entries))
                .await?;
        }
    }
    Ok(())
}

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

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(handler),
    )
    .dependencies(dptree::deps![repo])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
