use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use sqlx_core::types::chrono;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[sqlx(default)]
    pub avatar: Option<String>,
    pub is_delete: i8,
    pub create_at: chrono::NaiveDateTime,
}

impl User {
    pub async fn get_all(conn: &MySqlPool) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = sqlx::query_as!(User, "select * from users limit 10").fetch_all::<_>(conn).await?;
        Ok(rows)
    }
}