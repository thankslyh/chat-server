use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::model::message::MessageType;
use super::Delete;
use sqlx;
use sqlx::MySqlPool;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Default, sqlx::Type)]
pub enum ChatType {
    #[default]
    Single = 1,
    Group = 2,
    Other = 3,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, sqlx::FromRow)]
pub struct Conversation {
    pub id: Option<i64>,
    pub conv_id: String,
    pub chat_type: ChatType,
    pub nickname: Option<String>,
    pub remark: Option<String>,
    pub avatar: Option<String>,
    pub last_msg_type: MessageType,
    pub last_msg_content: String,
    pub owner: String,
    pub relate_ids: String,
    pub is_delete: Delete,
    pub create_at: chrono::DateTime<chrono::Local>,
    pub update_at: chrono::DateTime<chrono::Local>,
}

impl Conversation {
    pub async fn create(conn: &MySqlPool, conv: &mut Conversation) -> Result<(), Box<dyn Error>> {
        conv.create_at = chrono::Local::now();
        conv.update_at = conv.create_at;
        sqlx::query("insert into conversation (conv_id, chat_type, nickname, remark, avatar, last_msg_type, last_msg_content, owner, relate_ids, is_delete, create_at, update_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(conv.conv_id.clone())
            .bind(conv.chat_type.to_owned())
            .bind(conv.nickname.clone())
            .bind(conv.avatar.clone())
            .bind(conv.last_msg_type)
            .bind(conv.last_msg_content.clone())
            .bind(conv.owner.clone())
            .bind(conv.relate_ids.clone())
            .bind(conv.is_delete)
            .bind(conv.create_at)
            .bind(conv.update_at)
            .execute(conn)
            .await?;
        Ok(())
    }
}