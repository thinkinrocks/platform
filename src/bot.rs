use std::sync::Arc;

use askama::Template;
use chrono::{Local, Utc};
use chrono_english::{Dialect, parse_date_string};
use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    dispatching::{DefaultKey, HandlerExt, UpdateFilterExt},
    dptree,
    macros::BotCommands,
    payloads::SendMessageSetters,
    prelude::{Dispatcher, Requester, RequesterExt, ResponseResult},
    sugar::request::RequestLinkPreviewExt,
    types::{Message, ParseMode, Update},
};

use crate::{
    repository::Repository,
    templates::{Cart, Help, Me, Search, SingleEntry, errors},
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
    Reserve(String),
    Check(String),
    Tts(String),
}

async fn handler<R: Requester + Send>(
    bot: R,
    msg: Message,
    cmd: Command,
    repo: Arc<Repository>,
) -> Result<(), R::Err> {
    match cmd {
        Command::Introduce(_) | Command::Tts(_) => {
            bot.send_message(msg.chat.id, "Sorry! This command doesn't work yet. If you need it to work message @ktnlvr, he made the bot.").await?;
        }
        Command::Start | Command::Help => {
            bot.send_message(msg.chat.id, Help.render().unwrap())
                .disable_link_preview(true)
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
            bot.send_message(msg.chat.id, rendered).await?;
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

            bot.send_message(msg.chat.id, rendered).await?;
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

            bot.send_message(msg.chat.id, rendered).await?;
        }
        Command::Reserve(reservation) => {
            let reservation = reservation.trim();
            let Some((begin, end)) = reservation
                .split_once("/")
                .and_then(|(begin, end)| {
                    parse_date_string(begin, Local::now(), Dialect::Uk)
                        .ok()
                        .map(|begin| (begin, end))
                })
                .and_then(|(begin, end)| {
                    parse_date_string(end, begin, Dialect::Uk)
                        .ok()
                        .map(|end| (begin, end))
                })
            else {
                let rendered = errors::ReservationSyntax.render().unwrap();
                bot.send_message(msg.chat.id, rendered).await?;

                return Ok(());
            };

            let x = format!("{} {}", begin, end);

            bot.send_message(msg.chat.id, x).await?;
        }
    }
    Ok(())
}

pub async fn serve_bot(repo: Arc<Repository>) {
    let bot = Bot::from_env().parse_mode(ParseMode::Html);

    Dispatcher::<DefaultParseMode<Bot>, <Bot as Requester>::Err, DefaultKey>::builder(
        bot,
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(handler::<DefaultParseMode<Bot>>),
    )
    .dependencies(dptree::deps![repo])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
