use super::*;
use crate::model::Delete;
use sea_orm::ActiveValue::Set;

#[derive(Default, Debug, Deserialize, Serialize, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "friends")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub who_uid: String,
    pub relate_uid: String,
    pub avatar: Option<String>,
    pub is_delete: Delete,
    pub create_at: chrono::DateTime<chrono::Local>,
    pub update_at: chrono::DateTime<chrono::Local>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn get_friend_by_id(db: &DbConn, id: i64) -> anyhow::Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_friend_by_uid(
        db: &DbConn,
        who_uid: &str,
    ) -> anyhow::Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::WhoUid.contains(who_uid))
            .one(db)
            .await
    }

    pub async fn get_friend_list_by_uid(
        db: &DbConn,
        who_uid: &str,
    ) -> anyhow::Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::WhoUid.contains(who_uid))
            .all(db)
            .await
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn build(db: &DbConn, who_uid: &str, relate_id: &str) -> anyhow::Result<(), DbErr> {
        let who_user = ActiveModel {
            who_uid: Set(who_uid.to_string()),
            relate_uid: Set(relate_id.to_string()),
            ..Default::default()
        };
        let relate_user = ActiveModel {
            who_uid: Set(relate_id.to_string()),
            relate_uid: Set(who_uid.to_string()),
            ..Default::default()
        };
        match Entity::insert_many([who_user, relate_user]).exec(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
