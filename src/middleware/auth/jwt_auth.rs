use hmac::{Hmac, Mac};
use jwt::{Error, FromBase64, SignWithKey, SigningAlgorithm, ToBase64, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::borrow::Cow;
use std::ops::Deref;
use std::time::{Duration, SystemTime};
use anyhow::anyhow;

lazy_static! {
    static ref KEY: Hmac<Sha256> = Hmac::new_from_slice(b"ccccchat").unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt<'a> {
    pub email: Cow<'a, str>,
    pub expire_time: SystemTime,
    pub user_id: Cow<'a, str>,
}

impl<'a> Jwt<'a> {
    pub fn new(email: &'a str, user_id: &'a str) -> Self {
        let now = SystemTime::now() + Duration::from_secs(5 * 60);
        Jwt {
            email: Cow::Borrowed(email),
            user_id: Cow::Borrowed(user_id),
            expire_time: now,
        }
    }

    pub fn gen_token(&self) -> anyhow::Result<String> {
        let token_str = self.sign_with_key(KEY.deref())?;
        Ok(token_str)
    }

    pub fn verify_with_key<'b: 'a>(token_str: &'b str) -> anyhow::Result<Jwt<'b>> {
        let jwt = token_str.verify_with_key(KEY.deref())?;
        Ok(jwt)
    }
}

#[cfg(test)]
mod tests {
    use crate::middleware::jwt_auth::Jwt;

    #[test]
    fn test1() {
        let jwt = Jwt::new("thankslyh@gmail.com", "aksjchkashckhaskc");
        let token_str = jwt.gen_token().unwrap();
        println!("token_str is {}", token_str);
        let jwt = Jwt::verify_with_key(&token_str).unwrap();
        println!("jwt is {:#?}", jwt);
    }
}
