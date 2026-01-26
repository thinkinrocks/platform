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
    templates::{Cart, Help, Me, Search, SingleEntry},
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Me,
    Help,
    Start,
    Introduce(String),
    Search(String),
    Cart(String),
    Check(String),
    Tts(String),
}

async fn handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    repo: Arc<Repository>,
) -> ResponseResult<()> {
    match cmd {
        Command::Introduce(_) | Command::Tts(_) => {
            bot.send_message(msg.chat.id, "Sorry! This command doesn't work yet. If you need it to work message @ktnlvr, he made the bot.").await?;
        }
        Command::Start | Command::Help => {
            bot.send_message(msg.chat.id, Help.render().unwrap())
                .parse_mode(ParseMode::Html)
                .await
                .unwrap();
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
        Command::Check(id) => {
            let now = Utc::now().naive_utc();
            let entry = repo.get_entry(id.to_string()).await.unwrap().unwrap();
            let reserved = repo.is_entry_reserved(id, now, now).await.unwrap();

            let entry = SingleEntry {
                entry: &entry,
                reserved,
            };

            let rendered = entry.render().unwrap();

            bot.send_message(msg.chat.id, rendered)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Cart(ids) => {
            let ids = ids.split_whitespace().map(String::from).collect::<Vec<_>>();
            let username = msg.chat.username().unwrap();

            for id in ids {
                repo.add_to_cart(username, &id).await.unwrap();
            }

            let cart = repo.get_cart(username).await.unwrap();

            let cart = Cart { entries: &cart[..] };
            let rendered = cart.render().unwrap();

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
