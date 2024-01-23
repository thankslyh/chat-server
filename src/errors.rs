use self::CustomError::*;
use crate::routes::ServiceResponse;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub enum BusinessCode {
    Success = 200,
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("成功")]
    Success,
    #[error("内部错误，错误信息：`{0}`")]
    InternalServerError(&'static str),
    #[error("auth fail：`{0}`")]
    AuthFail(String),
    // 业务错误
    #[error("`{1}`")]
    BusinessError(BusinessCode, &'static str),
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            BusinessError(_, _) => StatusCode::OK,
            InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthFail(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            BusinessError(code, msg) => {
                HttpResponse::build(self.status_code()).json(ServiceResponse::<isize> {
                    code: *code as usize,
                    msg,
                    data: None,
                })
            }
            _ => HttpResponse::build(self.status_code()).finish(),
        };
    }
}
