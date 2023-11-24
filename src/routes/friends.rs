use crate::errors::CustomError;
use crate::routes::{CustomError, ServiceResponse};
use crate::{service, AppState};
use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;

const PREFIX: &'static str = "/friend";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddForm {
    uid: &'static str,
}

#[post("/add")]
pub async fn add(
    app_state: web::Data<AppState>,
    form: web::Form<AddForm>,
) -> actix_web::Result<impl Responder> {
    let db = &app_state.conn;
    let user = &app_state.user.unwrap();
    let exist = service::friend::relation_is_exist(db, user.uid.as_str()).await?;
    if exist {
        return web::Json(ServiceResponse {
            code: CustomError::BusinessFriendExist.into(),
            data: None,
            msg: CustomError::BusinessFriendExist.description(),
        });
    }
    service::friend::add_friend(db, user.uid.as_str(), form.uid).await?;
    web::Json(ServiceResponse {
        code: CustomError::Success,
        data: None,
        msg: "",
    })
}
