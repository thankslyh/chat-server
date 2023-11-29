mod friends;
pub mod user;

// pub mod conversation;
use crate::errors::CustomError;
use actix_web::{get, post, web, HttpResponse, HttpServer, Scope};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use Into;

#[derive(Debug, Serialize, Clone, Default)]
pub struct ServiceResponse<'a, T>
where
    T: Debug + Serialize,
{
    pub code: usize,
    pub data: Option<T>,
    pub msg: &'a str,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Pagination<T>
where
    T: Debug + Serialize,
{
    pub list: Vec<T>,
    pub total: u64,
}
