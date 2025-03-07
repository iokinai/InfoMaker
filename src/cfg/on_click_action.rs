use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OnClickAction {
    #[serde(rename = "set_text")]
    SetText(String),
    #[serde(rename = "switch_state")]
    SwitchState(String),
}
