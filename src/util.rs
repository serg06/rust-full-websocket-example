use serde::Serialize;
use tungstenite::Message;

pub fn get_msg_text(msg: &Message) -> Option<&str> {
    match msg {
        Message::Text(s) => Some(s),
        Message::Binary(v) => Some(std::str::from_utf8(v).expect("Invalid UTF8")),
        _ => None
    }
}

pub trait ToMessage {
    fn to_msg(&self) -> Message;
}

// Easily convert any json-serializable type to a tungstenite Message.
impl<T> ToMessage for T
    where
        T: ?Sized + Serialize,
{
    fn to_msg(&self) -> Message {
        Message::Text(serde_json::to_string(self).unwrap()) // TODO: Fix question mark
    }
}
