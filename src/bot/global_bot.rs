use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use teloxide::{
    dispatching::{
        UpdateHandler,
        dialogue::{self, GetChatId, InMemStorage},
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use trl::{getters, trl};

use crate::cfg::{Btn, ConfigState, OnClickAction};

use super::{BotState, database_state::DatabaseState, get_handler};

#[derive(trl)]
#[getters]
pub struct GlobalBot {
    bot: Bot,
    current_state: RwLock<String>,
    states: HashMap<String, DatabaseState>,
    btn_acts: HashMap<String, OnClickAction>,
}

impl GlobalBot {
    fn construct_kb(
        btns: &Vec<Btn>,
        i: usize,
        btn_acts: &mut HashMap<String, OnClickAction>,
    ) -> InlineKeyboardMarkup {
        let mut buttons = Vec::new();

        for (j, btn) in btns.iter().enumerate() {
            let btn_data = format!("{}btn{}", i, j);

            buttons.push(InlineKeyboardButton::new(
                btn.text().clone(),
                InlineKeyboardButtonKind::CallbackData(btn_data.clone()),
            ));

            btn_acts.insert(btn_data.clone(), btn.on_click().clone());
        }

        InlineKeyboardMarkup::new(vec![buttons])
    }

    pub fn new(bot: Bot, states: Vec<ConfigState>) -> GlobalBot {
        if states.is_empty() {
            panic!("You should have at least one state")
        }

        let mut states_map = HashMap::new();
        let mut btn_acts = HashMap::new();

        for (i, state) in states.iter().enumerate() {
            states_map.insert(
                state.name().clone(),
                DatabaseState::new(
                    GlobalBot::construct_kb(state.btns(), i, &mut btn_acts),
                    state.text().clone(),
                ),
            );
        }

        GlobalBot {
            bot,
            current_state: RwLock::new(states.get(0).unwrap().name().clone()),
            states: states_map,
            btn_acts,
        }
    }

    pub async fn run(self: Arc<Self>) {
        Dispatcher::builder(self.bot.clone(), get_handler())
            .dependencies(dptree::deps![self.clone(), InMemStorage::<BotState>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    /*

    Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<BotState>, BotState>()
                    .branch(dptree::case![BotState::Start].endpoint(Self::start))
                    .branch(Update::filter_callback_query().endpoint(Self::handle_callback)),

    */

    // async fn start(self: Arc<Self>, bot: Bot, dial: BotDialogue, msg: Message) -> HandlerResult {
    //     let state = self.states.get(&*self.current_state.read().await).unwrap();

    //     bot.send_message(msg.chat.id, state.text().read().await.clone())
    //         .reply_markup(state.kb().clone())
    //         .await?;

    //     dial.update(BotState::Database).await?;

    //     Ok(())
    // }

    // async fn handle_callback(
    //     self: Arc<Self>,
    //     bot: teloxide::Bot,
    //     dial: BotDialogue,
    //     q: CallbackQuery,
    // ) -> HandlerResult {
    //     println!("InsideHandleCallback");

    //     if let Some(data) = q.data {
    //         if let Some(act) = self.btn_acts.get(&data) {
    //             match act {
    //                 OnClickAction::SetText(text) => {
    //                     *self
    //                         .states
    //                         .get(&*self.current_state.read().await)
    //                         .unwrap()
    //                         .text()
    //                         .write()
    //                         .await = text.clone();

    //                     if let Some(ref message) = q.message {
    //                         self.states
    //                             .get(&*self.current_state.read().await)
    //                             .unwrap()
    //                             .edit_message(&self.bot, &message)
    //                             .await;
    //                     }
    //                 }
    //                 OnClickAction::SwitchState(name) => {
    //                     //let state = self.states.get(name).unwrap();

    //                     *self.current_state.write().await = name.clone();

    //                     if let Some(ref message) = q.message {
    //                         self.states
    //                             .get(&*self.current_state.read().await)
    //                             .unwrap()
    //                             .edit_message(&self.bot, &message)
    //                             .await;
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // pub async fn run(self: Arc<Self>) {
    //     Dispatcher::builder(
    //         self.bot.clone(),
    //         Update::filter_message()
    //             .branch(dptree::case![BotState::Start].endpoint(Self::start))
    //             .branch(Update::filter_callback_query().endpoint(Self::handle_callback)),
    //     )
    //     .dependencies(dptree::deps![self.clone(), InMemStorage::<BotState>::new()])
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;
    // }

    // async fn start(
    //     self: Arc<Self>,
    //     bot: teloxide::Bot,
    //     dial: BotDialogue,
    //     msg: Message,
    // ) -> HandlerResult {
    //     let keyboard = self
    //         .states
    //         .get(&*self.current_state.read().await)
    //         .unwrap()
    //         .kb();

    //     bot.send_message(msg.chat.id, "Ку, выбери вариант")
    //         .reply_markup(keyboard.clone())
    //         .await?;

    //     dial.update(BotState::Database).await?;

    //     Ok(())
    // }

    // async fn handle_callback(
    //     self: Arc<Self>,
    //     bot: teloxide::Bot,
    //     q: CallbackQuery,
    // ) -> HandlerResult {
    //     // if let Some(data) = q.data {
    //     //     bot.answer_callback_query(q.id).await?;
    //     //     bot.send_message(
    //     //         q.message.unwrap().chat.id,
    //     //         format!("Вы нажали: {}. Поле: {}", data, self.some_field),
    //     //     )
    //     //     .await?;
    //     // }

    //     Ok(())
    // }
}
