use actix_web::body::BoxBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse, Responder, ResponseError};
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::middleware::auth::jwt_auth::Jwt;
use crate::middleware::auth::validator::Validator;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::routes::ServiceResponse as BusinessResp;
pub mod jwt_auth;
pub mod validator;

pub struct Authoriser(Rc<Inner>);

pub struct Inner {
    white_list: Vec<&'static str>,
}

#[derive(Debug, ThisError)]
pub enum AuthError {
    #[error("获取 token 失败，token 不存在或格式错误, token:{0}")]
    GetToken(String),
    #[error("token 校验失败")]
    ValidateFail,
    #[error("token 失效，失效原因：{0}")]
    Invalidate(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            AuthError::GetToken(msg) => HttpResponse::Unauthorized().json(BusinessResp {
                code: 401,
                data: Some(msg),
                msg: "",
            }),
            AuthError::ValidateFail => HttpResponse::BadRequest().json(BusinessResp {
                code: 401,
                data: Some(""),
                msg: "token 校验失败",
            }),
            AuthError::Invalidate(_) => HttpResponse::BadRequest().json(BusinessResp {
                code: 401,
                data: Some(""),
                msg: "无效的 token",
            }),
        };
    }
}

impl Authoriser {
    pub fn new(white_list: Vec<&'static str>) -> Self {
        Authoriser(Rc::new(Inner { white_list }))
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authoriser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthoriserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthoriserMiddleware {
            service,
            white_list: self.0.white_list.clone(),
        }))
    }
}

pub struct AuthoriserMiddleware<S> {
    service: S,
    white_list: Vec<&'static str>,
}

impl<S> Validator for AuthoriserMiddleware<S> {}

impl<S, B> Service<ServiceRequest> for AuthoriserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        if self.white_list.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let output = fut.await?;
                Ok(output)
            });
        }
        let token = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .map(|t| t.to_str());
        if token.is_none() {
            return Box::pin(ready(Err(
                AuthError::GetToken("token不存在".to_string()).into()
            )));
        }
        let token = token.unwrap();
        let token = match token {
            Ok(token) => token,
            Err(e) => return Box::pin(ready(Err(AuthError::GetToken(e.to_string()).into()))),
        };
        let token = match token.strip_prefix("Bear ") {
            Some(token) => token,
            None => {
                return Box::pin(ready(Err(AuthError::GetToken(
                    "未获取到 token，请检查 token 格式".to_string(),
                )
                .into())))
            }
        };
        let jwt = match Jwt::verify_with_key(token) {
            Ok(jwt) => jwt,
            Err(e) => return Box::pin(ready(Err(AuthError::ValidateFail.into()))),
        };
        match self.validator(&jwt) {
            Err(_) => {
                return Box::pin(ready(Err(
                    AuthError::Invalidate("token 失效".to_string()).into()
                )));
            }
            _ => (),
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let output = fut.await?;
            Ok(output)
        })
    }
}
