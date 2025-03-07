use std::sync::Arc;

use teloxide::{
    Bot,
    dispatching::{UpdateFilterExt, UpdateHandler},
    dptree::{self},
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, Message, Update},
};

use crate::cfg::OnClickAction;

use super::GlobalBot;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn get_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        .branch(Update::filter_message().endpoint(display_start))
        .branch(Update::filter_callback_query().endpoint(cb_ep))
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

async fn cb_ep(gb: Arc<GlobalBot>, q: CallbackQuery) -> HandlerResult {
    if let Some(data) = q.data {
        if let Some(act) = gb.btn_acts().get(&data) {
            match act {
                OnClickAction::SetText(text) => {
                    *gb.states()
                        .get(&*gb.current_state().read().await)
                        .unwrap()
                        .text()
                        .write()
                        .await = text.clone();

                    if let Some(ref message) = q.message {
                        gb.states()
                            .get(&*gb.current_state().read().await)
                            .unwrap()
                            .edit_message(&gb.bot(), &message)
                            .await;
                    }
                }
                OnClickAction::SwitchState(name) => {
                    *gb.current_state().write().await = name.clone();

                    if let Some(ref message) = q.message {
                        gb.states()
                            .get(&*gb.current_state().read().await)
                            .unwrap()
                            .edit_message(&gb.bot(), &message)
                            .await;
                    }
                }
            }
        }
    }
    Ok(())
}
