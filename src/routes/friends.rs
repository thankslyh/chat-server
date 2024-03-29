use crate::errors::{BusinessCode, CustomError};
use crate::routes::ServiceResponse;
use crate::{service, AppState};
use actix_web::{post, web, Responder};
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};

const PREFIX: &'static str = "/friend";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddForm {
    uid: String,
}

#[post("/add")]
pub async fn add(
    app_state: web::Data<AppState>,
    db: web::Data<DbConn>,
    form: web::Form<AddForm>,
) -> actix_web::Result<impl Responder> {
    let user = &app_state.user.as_ref().unwrap();
    let exist = service::friend::relation_is_exist(&db, user.uid.as_str())
        .await
        .expect("");
    if exist {
        return Ok(web::Json(ServiceResponse {
            code: 100,
            data: Some(0),
            msg: "",
        }));
    }
    service::friend::add_friend(&db, user.uid.as_str(), &form.uid)
        .await
        .expect("");
    Ok(web::Json(ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(0),
        msg: "",
    }))
}

pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(PREFIX).service(add));
}
