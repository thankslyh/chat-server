use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use sqlx_core::row::Row;
use sqlx_core::types::chrono;

#[derive(Default, Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub uid: String,
    pub nickname: String,
    pub sex: i8,
    pub email: String,
    #[sqlx(default)]
    pub avatar: Option<String>,
    pub is_delete: i8,
    pub create_at: chrono::DateTime<chrono::Local>,
}

impl User {
    pub async fn get_all(conn: &MySqlPool) -> Result<Vec<User>, Box<dyn Error>> {
        let user = sqlx::query_as::<_, User>("select * from users")
            .fetch_all(conn)
            .await?;
        Ok(user)
    }

    pub async fn get_by_id(conn: &MySqlPool, id: u64) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query_as("select * from users where id = ?")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(user)
    }

    pub async fn create(conn: &MySqlPool, user: &User) -> Result<(), Box<dyn Error>> {
        sqlx::query("insert into users (uid, email, nickname, sex, is_delete, create_at) values (?, ?, ?, ?, ?, ?)")
            .bind(&user.uid)
            .bind(&user.email)
            .bind(&user.nickname)
            .bind(user.sex)
            .bind(user.is_delete)
            .bind(user.create_at)
            .execute(conn)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_gen_uuid() {
        let uid = uuid::Uuid::new_v4();
        println!("uuid:{}", uid);
    }
}