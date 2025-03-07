use serde::{Deserialize, Serialize};
use trl::{getters, trl};

use super::btn::Btn;

#[derive(trl, Serialize, Deserialize, Debug)]
#[getters]
pub struct ConfigState {
    name: String,
    btns: Vec<Btn>,
    text: String,
}
