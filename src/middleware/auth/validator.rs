use crate::middleware::auth::jwt_auth::Jwt;
use anyhow::{anyhow, Error};
use std::time::SystemTime;

pub trait Validator {
    fn validator(&self, jwt: &Jwt) -> anyhow::Result<()> {
        let now = SystemTime::now();
        if jwt.expire_time > now {
            return Err(Error::msg("token已过期"));
        }
        Ok(())
    }
}
