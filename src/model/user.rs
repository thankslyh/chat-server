use std::fmt::{Display, Formatter};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryOrder;
use sqlx_core::types::chrono;
use crate::model::Delete;
use super::*;

#[derive(Default, Debug, Deserialize, Serialize, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub uid: String,
    pub nickname: String,
    pub sex: Sex,
    pub email: String,
    pub avatar: Option<String>,
    pub is_delete: Delete,
    pub create_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, EnumIter, DeriveActiveEnum, PartialEq, Serialize, Deserialize, Clone)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum Sex {
    #[default]
    #[sea_orm(num_value = 0)]
    UnSet = 0,
    #[sea_orm(num_value = 1)]
    Woman = 1,
    #[sea_orm(num_value = 2)]
    Man = 2,
    #[sea_orm(num_value = 3)]
    UnKnow = 3,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn get_user_list(db: &DbConn, page: u64, page_per: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_asc(Column::Id)
            .paginate(db, page_per);
        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn get_user_by_id(db: &DbConn, id: u64) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, user: &Model) -> Result<ActiveModel, DbErr> {
        ActiveModel {
            email: Set(user.email.to_owned()),
            uid: Set(uuid::Uuid::new_v4().to_string()),
            nickname: Set(user.nickname.to_owned()),
            sex: Set(user.sex.to_owned()),
            ..Default::default()
        }
            .save(db)
            .await
    }

    pub async fn update_user_by_id(db: &DbConn, id: u64, user: &Model) -> Result<Model, DbErr> {
        let tmp_user = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("未找到 id".to_owned()))
            .map(Into::into)?;

        ActiveModel {
            id: Set(user.id),
            nickname: Set(user.nickname.to_owned()),
            sex: Set(user.sex.to_owned()),
            ..tmp_user
        }
            .update(db)
            .await
    }
}
