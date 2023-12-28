use self::CustomError::*;
use crate::routes::ServiceResponse;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("成功")]
    Success,
    #[error("内部错误，错误信息：`{0}`")]
    InternalServerError(String),
    #[error("auth fail：`{0}`")]
    AuthFail(String),
    // 业务错误
    #[error("`{0}`")]
    CommonBusinessError(String),
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthFail(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        if self.status_code() == StatusCode::OK {
            let code: usize = self.into();
            HttpResponse::build(self.status_code()).json(ServiceResponse::<isize> {
                code,
                msg: self.to_string().as_str(),
                data: None,
            })
        } else {
            HttpResponse::build(self.status_code()).finish()
        }
    }
}
impl From<CustomError> for usize {
    fn from(value: CustomError) -> Self {
        match value {
            Success => 200,
            InternalServerError(_) => 500,
            AuthFail(_) => 401,
            CommonBusinessError(_) => 100_101,
        }
    }
}

impl From<&CustomError> for usize {
    fn from(value: &CustomError) -> Self {
        match value {
            Success => 200,
            InternalServerError(_) => 500,
            AuthFail(_) => 401,
            CommonBusinessError(_) => 100_101,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::CustomError;

    #[test]
    fn test_into() {
        let e = &CustomError::AuthFail("用户不存在".to_string());
        let val: usize = e.into();
        assert_eq!(401, val)
    }
}
