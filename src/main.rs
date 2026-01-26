mod models;
mod repository;
mod templates;

use std::sync::Arc;

use askama::Template;
use chrono::Utc;
use sqlx::sqlite::SqlitePoolOptions;
use teloxide::{macros::BotCommands, prelude::*, types::ParseMode};

use crate::{
    repository::Repository,
    templates::{Me, Search, SingleEntry},
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Me,
    Help,
    Start,
    Introduce(String),
    Search(String),
    Reserve(String),
    Check(String),
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

            let me = Me { me: &user };

            bot.send_message(msg.chat.id, me.render().unwrap())
                .await
                .unwrap();
        }
        Command::Search(query) => {
            let query = query.as_str();
            let limit = 15;
            let entries = repo.search_entries(query, limit).await.unwrap();

            let search = Search {
                query,
                limit,
                entries: &entries[..],
            };

            let rendered = search.render().unwrap();
            bot.send_message(msg.chat.id, rendered)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Reserve(reservations) => todo!(),
        Command::Check(id) => {
            let entry = repo.get_entry(id.to_string()).await.unwrap().unwrap();
            let reserved = repo
                .is_entry_reserved(id, Utc::now().naive_utc())
                .await
                .unwrap();

            let entry = SingleEntry {
                entry: &entry,
                reserved,
            };

            let rendered = entry.render().unwrap();

            bot.send_message(msg.chat.id, rendered)
                .parse_mode(ParseMode::Html)
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
