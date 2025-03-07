use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OnClickAction {
    SetText(String),
    SwitchState(String),
}
