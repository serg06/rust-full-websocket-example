pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgInEchoData {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgOutEchoData {
    pub message: String,
}
