use super::*;
use crate::model::Delete;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryOrder;
use sqlx_core::types::chrono;
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Deserialize, Serialize, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub uid: String,
    pub nickname: String,
    #[sea_orm(column_type = "TinyInteger")]
    pub sex: Sex,
    pub email: String,
    pub avatar: Option<String>,
    #[sea_orm(column_type = "TinyInteger")]
    pub is_delete: Delete,
    pub update_at: chrono::DateTime<chrono::Local>,
    pub create_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, EnumIter, DeriveActiveEnum, PartialEq, Serialize, Deserialize, Clone)]
#[sea_orm(rs_type = "i8", db_type = "TinyInteger", enum_name = "sex")]
pub enum Sex {
    #[default]
    UnSet = 0,
    Woman = 1,
    Man = 2,
    UnKnow = 3,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn get_user_list(
        db: &DbConn,
        page: u64,
        page_per: u64,
    ) -> anyhow::Result<(Vec<Model>, u64)> {
        let paginator = Entity::find()
            .order_by_asc(Column::Id)
            .paginate(db, page_per);
        let num_pages = paginator.num_pages().await?;
        let res = paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, num_pages))?;
        Ok(res)
    }

    pub async fn get_user_by_id(db: &DbConn, id: u64) -> anyhow::Result<Option<Model>> {
        let res = Entity::find_by_id(id).one(db).await?;
        Ok(res)
    }

    pub async fn search_by_nickname(db: &DbConn, nickname: &str) -> anyhow::Result<Vec<Model>> {
        let res = Entity::find()
            .filter(Column::Nickname.contains(nickname))
            .all(db)
            .await?;
        Ok(res)
    }

    pub async fn get_by_email(db: &DbConn, email: &str) -> anyhow::Result<Vec<Model>> {
        let res = Entity::find()
            .filter(Column::Email.contains(email))
            .all(db)
            .await?;
        Ok(res)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, user: &Model) -> anyhow::Result<ActiveModel> {
        let model = ActiveModel {
            email: Set(user.email.to_owned()),
            uid: Set(uuid::Uuid::new_v4().to_string()),
            nickname: Set(user.nickname.to_owned()),
            sex: Set(user.sex.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?;
        Ok(model)
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: u64,
        user: &Model,
    ) -> anyhow::Result<Model, DbErr> {
        let mut tmp_user = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("未找到 id".to_owned()))
            .map::<ActiveModel, _>(Into::into)?;
        tmp_user.nickname = Set(user.nickname.to_owned());
        tmp_user.sex = Set(user.sex.to_owned());
        tmp_user.update_at = Set(chrono::Local::now());
        tmp_user.update(db).await
    }
}

#[cfg(test)]
mod tests {
    use crate::model::user::{ActiveModel, Model, Mutation, Query, Sex};
    use sea_orm::{ActiveEnum, Database};

    #[tokio_macros::test]
    async fn test_create() {
        let db = Database::connect("mysql://root@localhost:3306/chat")
            .await
            .unwrap();
        let res = Query::get_user_by_id(&db, 1).await.expect("");
        println!("{:#?}", res);
        println!("{:#?}", Sex::Man.to_value())
    }
}
