use super::*;
use crate::model::user::Sex;
use crate::{model, service, AppState};
use actix_web::Responder;
use sea_orm::TryIntoModel;
use serde::Deserialize;

const PREFIX: &'static str = "/user";
#[get("/list")]
async fn user_list(app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let list = crate::service::user::get_list(&app_state.conn)
        .await
        .expect("");
    let data = Pagination::<model::user::Model> {
        list: list.0,
        total: list.1,
    };
    let res = ServiceResponse {
        code: CustomError::Success,
        data,
        msg: "",
    };
    Ok(web::Json(res))
}

#[get("/detail/{id}")]
async fn user_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(u32,)>,
) -> actix_web::Result<impl Responder> {
    println!("{:#?}", params);
    let res = service::user::get_user_by_id(&app_state.conn, params.0 as u64)
        .await
        .expect("");
    let res = ServiceResponse::<Option<model::user::Model>> {
        code: CustomError::Success,
        data: res,
        msg: "",
    };
    println!("{:#?}", res);
    Ok(web::Json(res))
}

#[derive(Deserialize, Clone)]
pub struct Acc {
    email: String,
    nickname: Option<String>,
}
#[post("/create")]
async fn user_create(
    app_state: web::Data<AppState>,
    info: web::Form<Acc>,
) -> actix_web::Result<impl Responder> {
    let mut user = model::user::Model::default();
    user.email = info.email.to_owned();
    user.nickname = info.nickname.clone().unwrap_or(user.email.clone());
    let tmp_res = service::user::create(&app_state.conn, &user)
        .await
        .unwrap()
        .try_into_model()
        .expect("");
    let res = ServiceResponse {
        code: CustomError::Success,
        data: tmp_res,
        msg: "",
    };
    Ok(web::Json(res))
}

#[derive(Deserialize)]
struct UpdatedInfo {
    sex: Sex,
    avatar: Option<String>,
    nickname: Option<String>,
}
#[post("/update/{id}")]
async fn user_update(
    app_state: web::Data<AppState>,
    params: web::Path<(u32,)>,
    query: web::Json<UpdatedInfo>,
) -> actix_web::Result<impl Responder> {
    let tmp_user = model::user::Model {
        sex: query.sex.clone(),
        nickname: query.nickname.clone().unwrap(),
        avatar: query.avatar.clone(),
        ..Default::default()
    };
    let res = model::user::Mutation::update_user_by_id(&app_state.conn, params.0 as u64, &tmp_user)
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
    app_state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> actix_web::Result<impl Responder> {
    let list = service::user::search(&app_state.conn, query.keyword.as_str())
        .await
        .expect("");
    let res = ServiceResponse {
        code: CustomError::Success,
        data: list,
        msg: "",
    };
    Ok(web::Json(res))
}

pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(PREFIX)
            .service(user_list)
            .service(user_detail)
            .service(user_create)
            .service(user_update)
            .service(user_delete)
            .service(user_search),
    );
}
