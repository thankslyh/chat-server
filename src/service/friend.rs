use crate::model;
use sea_orm::DbConn;

pub async fn relation_is_exist(db: &DbConn, who_uid: &str) -> anyhow::Result<bool> {
    let res = model::friend::Query::get_friend_by_uid(db, who_uid).await?;
    Ok(res.is_some())
}

pub async fn add_friend(db: &DbConn, who_uid: &str, relate_uid: &str) -> anyhow::Result<()> {
    Ok(model::friend::Mutation::build(db, who_uid, relate_uid).await?)
}
