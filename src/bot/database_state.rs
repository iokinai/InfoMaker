use tokio::sync::RwLock;

use teloxide::{
    Bot,
    payloads::EditMessageReplyMarkupSetters,
    prelude::Requester,
    types::{InlineKeyboardMarkup, MaybeInaccessibleMessage},
};
use trl::{getters, trl};

#[derive(trl)]
#[getters]
pub struct DatabaseState {
    kb: InlineKeyboardMarkup,
    text: RwLock<String>,
}

impl DatabaseState {
    pub fn new(kb: InlineKeyboardMarkup, text: String) -> DatabaseState {
        DatabaseState {
            kb,
            text: RwLock::new(text),
        }
    }

    pub async fn edit_message(&self, bot: &Bot, message: &MaybeInaccessibleMessage) {
        let text = self.text.read().await.clone();
        let kb = self.kb.clone();

        bot.edit_message_text(message.chat().id, message.id(), text)
            .await
            .unwrap();

        bot.edit_message_reply_markup(message.chat().id, message.id())
            .reply_markup(kb)
            .await
            .unwrap();
    }
}
