use sea_orm::prelude::ChronoDateTimeLocal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use ContentType::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextContent(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageContent {
    pub width: usize,
    pub height: usize,
    pub origin_url: String,
    pub thumb_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, sqlx::Type)]
pub enum ContentType {
    #[default]
    Text = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub content_type: ContentType,
    pub content: String,
    pub send_time: ChronoDateTimeLocal,
    pub nickname: String,
}

impl FromStr for Message {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = serde_json::from_str::<Self>(s);
        if let Ok(res) = res {
            return Ok(res);
        }
        Err(())
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Text => write!(f, "1 - 文本消息"),
        }
    }
}
