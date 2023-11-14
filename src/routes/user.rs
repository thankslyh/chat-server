use actix_web::Responder;
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::{AppState, db, model};
use super::*;

const PREFIX: &'static str = "/user";
#[get("/list")]
async fn user_list(app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let list = crate::db::user::get_list(&app_state.conn).await.expect("");
    Ok(web::Json(list))
}

#[get("/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", path);
    HttpResponse::Ok().body("test")
}

#[derive(Deserialize)]
pub struct Acc {
    email: String,

}
#[post("/create")]
async fn user_create(app_state: web::Data<AppState>, info: web::Form<Acc>) -> actix_web::Result<impl Responder> {
    let mut user = model::user::Model::default();
    user.email = info.email.to_owned();
    let res = db::user::create(&app_state.conn, &user).await.expect("");
    Ok(web::Json(user.clone()))
}

#[post("/update/{id}")]
async fn user_update(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

#[post("/delete/{id}")]
async fn user_delete(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(PREFIX)
            .service(user_list)
            .service(user_detail)
            .service(user_create)
            .service(user_update)
            .service(user_delete)
    );
}