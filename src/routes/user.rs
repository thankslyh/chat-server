use actix_web::Responder;
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::db;
use crate::model::user::User;
use super::*;

const PREFIX: &'static str = "/user";
#[get("/list")]
async fn user_list(pool: web::Data<MySqlPool>) -> actix_web::Result<impl Responder> {
    let list = crate::db::user::get_list(&pool).await?;
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
async fn user_create(pool: web::Data<MySqlPool>, info: web::Form<Acc>) -> actix_web::Result<impl Responder> {
    let mut user = User::default();
    let uid = uuid::Uuid::new_v4();
    user.email = info.email.to_owned();
    user.uid = uid.to_owned().to_string();
    db::user::create(&pool, &mut user).await;
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