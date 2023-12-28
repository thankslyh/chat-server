use crate::model::user;
use sea_orm::DbConn;

pub async fn get_list(conn: &DbConn) -> anyhow::Result<(Vec<user::Model>, u64)> {
    user::Query::get_user_list(conn, 1, 10).await
}

pub async fn create(conn: &DbConn, user: &user::Model) -> anyhow::Result<user::ActiveModel> {
    user::Mutation::create_user(conn, user).await
}

pub async fn get_user_by_id(conn: &DbConn, id: u64) -> anyhow::Result<Option<user::Model>> {
    user::Query::get_user_by_id(conn, id).await
}

pub async fn search(conn: &DbConn, keyword: &str) -> anyhow::Result<Vec<user::Model>> {
    user::Query::search_by_nickname(conn, keyword).await
}
