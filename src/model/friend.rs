use crate::model::Delete;
use super::*;

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
    async fn get_friend_by_id(db: &DbConn, id: i64) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    async fn get_friend_by_uid(db: &DbConn, who_uid: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::WhoUid.contains(who_uid))
            .one(db)
            .await
    }
}

pub struct Mutation;

impl Mutation {
}

