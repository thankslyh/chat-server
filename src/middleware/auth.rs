use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::service;
use actix_web::Error;
use std::future::{ready, Ready};
use std::rc::Rc;

use futures_util::future::LocalBoxFuture;

pub struct Auther(Rc<Inner>);

pub struct Inner {
    key: String,
}

impl Auther {
    pub fn new(key: String) -> Self {
        Auther(Rc::new(Inner { key }))
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auther
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = AutherMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AutherMiddleware { service: S }))
    }
}

pub struct AutherMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AutherMiddleware<S>
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
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
