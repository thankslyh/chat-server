use redis::{RedisConnectionInfo, RedisError, RedisResult};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::fmt::Display;
use std::time;

pub mod friend;
pub mod user;
// pub mod conversation;

pub async fn init(data_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(data_url).await
}

pub async fn init_redis(data_url: &str) -> RedisResult<redis::Client> {
    let client = redis::Client::open(data_url)?;
    Ok(client)
}

pub struct RedisKey<'a>(pub Vec<&'a str>);

impl<'a> Display for RedisKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(":"))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::RedisKey;
    use std::time;

    #[test]
    fn test_redis_key() {
        let key: RedisKey = RedisKey(vec!["a", "b", "c"]);
        let str = key.to_string();
        println!("str is {}", str);
        let now = chrono::Local::now();
        println!("{:?}", now);
        let now = now + time::Duration::from_secs(5 * 60);
        println!("{:?}", now);
    }
}
