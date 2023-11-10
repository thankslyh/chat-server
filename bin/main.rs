use std::collections::HashMap;
use actix_web::{App, delete, HttpServer, middleware, web};
use chat_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    let pool = chat_server::db::init("mysql://root@localhost:3306/chat").await.unwrap();
    let user = chat_server::model::user::User::default();
    let mut context = chat_server::Context {
        user
    };
    HttpServer::new(move || {
        App::new()
            .configure(
                chat_server::routes::user::entry,
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(context.clone()))
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 10001))?
        .run()
        .await
}