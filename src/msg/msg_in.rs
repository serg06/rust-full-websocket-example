use serde::{Deserialize, Serialize};
use crate::msg::msg_echo::MsgInEchoData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum MsgIn {
    Echo(MsgInEchoData)
}
