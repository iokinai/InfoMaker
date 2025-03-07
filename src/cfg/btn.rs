use serde::{Deserialize, Serialize};
use trl::{getters, trl};

use super::on_click_action::OnClickAction;

#[derive(Serialize, Deserialize, Debug, trl)]
#[getters]
pub struct Btn {
    text: String,
    on_click: OnClickAction,
}
