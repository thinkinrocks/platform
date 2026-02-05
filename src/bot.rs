use std::sync::Arc;

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

use crate::repository::Repository;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Me,
    Help,
    Start,
}

async fn handler<R: Requester + Send>(
    bot: R,
    msg: Message,
    cmd: Command,
    repo: Arc<Repository>,
) -> Result<(), R::Err> {
    match cmd {
        Command::Start | Command::Help => {
            bot.send_message(msg.chat.id, ":3")
                .disable_link_preview(true)
                .await
                .unwrap();
        }
        Command::Me => {
            bot.send_message(msg.chat.id, "Penis").await.unwrap();
        }
    };

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
