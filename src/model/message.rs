use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use MessageType::*;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Default, sqlx::Type)]
pub enum MessageType {
    #[default]
    Text = 1,
    At = 2
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Text => write!(f, "1 - 文本消息"),
            At => write!(f, "2 - 艾特消息")
        }
    }
}