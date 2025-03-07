use std::sync::Arc;

use teloxide::{
    Bot,
    dispatching::{
        HandlerExt, UpdateFilterExt, UpdateHandler,
        dialogue::{self, InMemStorage},
    },
    dptree::{self, di::Asyncify},
    payloads::SendMessageSetters,
    prelude::{Dialogue, Requester},
    types::{CallbackQuery, Message, Update},
};

use super::{BotCommand, BotState, GlobalBot};

type BotDialogue = Dialogue<BotState, InMemStorage<BotState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn get_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<BotState>, BotState, _>()
        .branch(Update::filter_message().endpoint(display_start))
        .branch(
            Update::filter_callback_query()
                .filter_map(|query: CallbackQuery| {
                    println!("Callback query received: {:?}", query.data);
                    query.data.clone()
                })
                .endpoint(|data: String| async move {
                    println!("Handling callback with data: {}", data);
                    Ok(())
                }),
        )
}

async fn display_start(gb: Arc<GlobalBot>, bot: Bot, msg: Message) -> HandlerResult {
    let current = gb
        .states()
        .get(&*gb.current_state().read().await.clone())
        .unwrap();

    bot.send_message(msg.chat.id, current.text().read().await.clone())
        .reply_markup(current.kb().clone())
        .await?;

    Ok(())
}

async fn cb_ep(gb: Arc<GlobalBot>, bot: Bot, q: CallbackQuery) -> HandlerResult {
    //println!("callback endpoint");

    Ok(())
}
