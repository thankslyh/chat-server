use super::*;
use crate::email::Mail;
use crate::errors::BusinessCode;
use crate::middleware::Jwt;
use crate::model::user::Sex;
use crate::service::RedisKey;
use crate::{model, service, utils, AppState};
use actix_web::Responder;
use futures_util::TryFutureExt;
use sea_orm::{DbConn, DbErr, TryIntoModel};
use serde::Deserialize;
use std::time;

const PREFIX: &'static str = "/user";
#[get("/list")]
async fn user_list(db: web::Data<DbConn>) -> actix_web::Result<impl Responder> {
    let list =
        crate::service::user::get_list(&db)
            .await
            .map_err(|e| match e.downcast_ref::<DbErr>() {
                Some(_) => CustomError::InternalServerError("内部错误"),
                None => CustomError::BusinessError(BusinessCode::Success, "获取好友列表失败"),
            })?;
    let data = Pagination::<model::user::Model> {
        list: list.0,
        total: list.1,
    };
    let res = ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(data),
        msg: "",
    };
    Ok(web::Json(res))
}

#[get("/detail/{id}")]
async fn user_detail(
    db: web::Data<DbConn>,
    params: web::Path<(u32,)>,
) -> actix_web::Result<impl Responder> {
    println!("{:#?}", params);
    let res = service::user::get_user_by_id(&db, params.0 as u64)
        .await
        .expect("");
    let res = ServiceResponse::<Option<model::user::Model>> {
        code: BusinessCode::Success as usize,
        data: Some(res),
        msg: "",
    };
    println!("{:#?}", res);
    Ok(web::Json(res))
}

#[derive(Deserialize, Clone)]
pub struct Acc {
    email: String,
    nickname: Option<String>,
    code: Option<String>,
}
#[post("/create")]
async fn user_create(
    db: web::Data<DbConn>,
    info: web::Form<Acc>,
) -> actix_web::Result<impl Responder> {
    let mut user = model::user::Model::default();
    user.email = info.email.to_owned();
    user.nickname = info.nickname.clone().unwrap_or(user.email.clone());
    let tmp_res = service::user::create(&db, &user)
        .await
        .unwrap()
        .try_into_model()
        .expect("");
    let res = ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(tmp_res),
        msg: "",
    };
    Ok(web::Json(res))
}

#[post("/login")]
async fn user_login(
    db: web::Data<DbConn>,
    body: web::Json<Acc>,
) -> actix_web::Result<impl Responder> {
    let email = &body.email;
    let res = service::user::get_user_by_email(&db, &email)
        .await
        .map_err(|err| CustomError::InternalServerError("内部错误"))?;
    if let Some(user) = res.first() {
        let token = Jwt::new(&user.email, &user.uid)
            .gen_token()
            .map_err(|e| CustomError::InternalServerError("内部错误"))?;
        return Ok(web::Json(ServiceResponse {
            code: BusinessCode::Success as usize,
            data: Some(token),
            msg: "",
        }));
    }
    let mut user = model::user::Model::default();
    user.nickname = email.clone();
    user.email = email.clone();
    let user = service::user::create(&db, &user)
        .await
        .map_err(|e| CustomError::InternalServerError("内部错误"))?
        .try_into_model()
        .map_err(|e| CustomError::InternalServerError("内部错误"))?;
    let token = Jwt::new(&user.email, &user.uid)
        .gen_token()
        .map_err(|e| CustomError::InternalServerError("内部错误"))?;
    Ok(web::Json(ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(token),
        msg: "",
    }))
}

#[derive(Deserialize)]
struct UpdatedInfo {
    sex: Sex,
    avatar: Option<String>,
    nickname: Option<String>,
}
#[post("/update/{id}")]
async fn user_update(
    db: web::Data<DbConn>,
    params: web::Path<(u32,)>,
    query: web::Json<UpdatedInfo>,
) -> actix_web::Result<impl Responder> {
    let tmp_user = model::user::Model {
        sex: query.sex.clone(),
        nickname: query.nickname.clone().unwrap(),
        avatar: query.avatar.clone(),
        ..Default::default()
    };
    let res = model::user::Mutation::update_user_by_id(&db, params.0 as u64, &tmp_user)
        .await
        .expect("");
    Ok(web::Json(res.clone()))
}

#[post("/delete/{id}")]
async fn user_delete(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    keyword: String,
}
#[get("/search")]
async fn user_search(
    db: web::Data<DbConn>,
    query: web::Query<SearchQuery>,
) -> actix_web::Result<impl Responder> {
    let list = service::user::search(&db, query.keyword.as_str())
        .await
        .expect("");
    let res = ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(list),
        msg: "",
    };
    Ok(web::Json(res))
}

#[post("/verify-code")]
async fn send_verify_code(
    rds: web::Data<redis::Client>,
    body: web::Form<Acc>,
) -> actix_web::Result<impl Responder> {
    let email = body.email.clone();
    let email_server = Mail::new();
    let code = utils::functions::gen_random_code(6);
    let _ = email_server
        .send_text(email.clone(), code.clone())
        .map_err(|e| {
            CustomError::BusinessError(BusinessCode::Success, "邮箱验证码发送失败，请重试~")
        })?;
    let mut conn = rds
        .get_connection_manager()
        .await
        .map_err(|e| CustomError::InternalServerError("内部错误"))?;
    let res = redis::Cmd::mset(&[(
        RedisKey(vec!["verify_code", &email]).to_string(),
        RedisKey(vec![
            &code,
            &utils::functions::gen_expire_time(chrono::Duration::minutes(5)).to_string(),
        ])
        .to_string(),
    )])
    .query_async::<_, String>(&mut conn)
    .await
    .map_err(|e| CustomError::InternalServerError("内部错误"))?;
    Ok(web::Json(ServiceResponse {
        code: BusinessCode::Success as usize,
        data: Some(""),
        msg: "",
    }))
}

pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(PREFIX)
            .service(user_list)
            .service(user_detail)
            .service(user_create)
            .service(user_update)
            .service(user_delete)
            .service(user_search)
            .service(user_login)
            .service(send_verify_code),
    );
}
