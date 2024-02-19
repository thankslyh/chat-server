#[macro_use]
extern crate lazy_static;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use futures_util::lock::Mutex;
pub use redis::aio::ConnectionLike;
use std::env;
use std::sync::Arc;

mod email;
pub mod errors;
pub mod middleware;
pub mod model;
pub mod routes;
pub mod service;
pub mod socket;
mod utils;

#[derive(Debug)]
pub struct CtxUser {
    pub id: u64,
    pub uid: String,
}

#[derive(Debug)]
pub struct AppState {
    pub user: Option<CtxUser>,
}

pub async fn start() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("数据库 url 不存在");
    let redis_url = env::var("REDIS_URL").expect("redis url 不存在");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = service::init(&database_url)
        .await
        .expect("db connect error");
    let rds_client = service::init_redis(&redis_url)
        .await
        .expect("redis connect error");
    let user = crate::CtxUser {
        id: 1,
        uid: "caf1577c-8029-4594-aa76-3915a9719f6c".to_string(),
    };
    let state = Arc::new(Mutex::new(crate::AppState { user: Some(user) }));

    let serv = crate::socket::ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(crate::middleware::Authoriser::new(vec![
                "/user/login",
                "/user/verify-code",
            ]))
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(rds_client.clone()))
            .app_data(web::Data::new(serv.clone()))
            .configure(crate::routes::user::entry)
            .configure(crate::routes::friends::entry)
            .route("/ws", web::get().to(crate::socket::start))
    })
    .bind(("127.0.0.1", 10001))?
    .run()
    .await
}
