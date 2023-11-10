use std::time;
use sqlx::mysql::{ MySqlPoolOptions, MySqlPool };

pub mod user;
pub mod conversation;

pub async fn init(data_url: &str) -> Result<MySqlPool, sqlx_core::error::Error> {
    MySqlPoolOptions::new()
        .acquire_timeout(time::Duration::from_secs(1))
        .connect(data_url)
        .await
}