use crate::errors::CustomError;
use crate::model;
use sea_orm::DbConn;
use std::error::Error;

pub async fn relation_is_exist(db: &DbConn, who_uid: &str) -> Result<bool, CustomError> {
    let res = model::friend::Query::get_friend_by_uid(db, who_uid)
        .await
        .map_err(|e| CustomError::InternalServerError(e.to_string()))?;
    Ok(res.is_some())
}

pub async fn add_friend(db: &DbConn, who_uid: &str, relate_uid: &str) -> Result<(), impl Error> {
    model::friend::Mutation::build(db, who_uid, relate_uid).await
}
