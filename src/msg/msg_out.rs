use serde::{Deserialize, Serialize};
use crate::msg::msg_echo::MsgOutEchoData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum MsgOut {
    Echo(MsgOutEchoData)
}
