use self::CustomError::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CustomError {
    Success,
    InternalServerError(String),
    AuthFail(&'static str),
    // 业务错误
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

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Success => write!(f, "success"),
            AuthFail(s) => write!(f, "auth fail:{}", s),
            InternalServerError(s) => write!(f, "内部服务错误：{}", s),
            BusinessFriendExist => write!(f, "业务错误"),
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
