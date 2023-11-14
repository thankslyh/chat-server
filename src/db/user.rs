use std::error::Error;
use sea_orm::DbConn;
use crate::model::user;

pub async fn get_list(conn: &DbConn) -> Result<(Vec<user::Model>, u64), impl Error> {
    user::Query::get_user_list(conn, 1, 10).await
}

pub async fn create(conn: &DbConn, user: &user::Model) -> Result<user::ActiveModel, impl Error> {
    user::Mutation::create_user(conn, user).await
}