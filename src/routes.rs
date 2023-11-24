mod friends;
pub mod user;

// pub mod conversation;
use crate::errors::CustomError;
use actix_web::{get, post, web, HttpResponse, HttpServer, Scope};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use Into;

#[derive(Debug, Serialize, Clone)]
pub struct ServiceResponse<'a, T>
where
    T: Debug + Serialize,
{
    pub code: CustomError::BusinessFriendExist,
    pub data: T,
    msg: &'a str,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Pagination<T>
where
    T: Debug + Serialize,
{
    pub list: Vec<T>,
    pub total: u64,
}
