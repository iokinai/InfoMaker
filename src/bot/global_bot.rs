use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup},
};
use trl::{getters, trl};

use crate::cfg::{Btn, ConfigState, OnClickAction};

use super::{database_state::DatabaseState, get_handler};

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
            .dependencies(dptree::deps![self.clone()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
