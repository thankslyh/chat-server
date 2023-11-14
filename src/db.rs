use std::time;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub mod user;
// pub mod conversation;

pub async fn init(data_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(data_url).await
}