use self::CustomError::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("成功")]
    Success,
    #[error("内部错误")]
    InternalServerError(String),
    #[error("auth fail")]
    AuthFail(&'static str),
    // 业务错误
    #[error("好友信息已经存在")]
    BusinessFriendExist,
}

impl From<CustomError> for usize {
    fn from(value: CustomError) -> Self {
        match value {
            Success => 200,
            InternalServerError(_) => 500,
            AuthFail(_) => 401,
            BusinessFriendExist => 100_101,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::CustomError;

    #[test]
    fn test_into() {
        let e = CustomError::AuthFail("用户不存在");
        let val: usize = e.into();
        assert_eq!(401, val)
    }
}
