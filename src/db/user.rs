use std::error::Error;
use actix_web::cookie::Expiration::DateTime;
use chrono::{Local, NaiveDateTime, Utc};
use sqlx::MySqlPool;
use crate::model::user::User;

pub async fn get_list(conn: &MySqlPool) -> Result<Vec<User>, Box<dyn Error>> {
    User::get_all(conn).await
}

pub async fn create(conn: &MySqlPool, user: &mut User) -> Result<(), Box<dyn Error>> {
    user.is_delete = 0;
    user.create_at = Local::now();
    User::create(conn, user).await
}