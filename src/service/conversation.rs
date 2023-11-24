use sqlx::MySqlPool;
use std::error::Error;
use crate::model::conversation::Conversation;

async fn create_conv(pool: &MySqlPool, conv: &mut Conversation) -> Result<(), Box<dyn Error>> {
    Conversation::create(pool, conv).await
}