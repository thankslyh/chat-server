use actix_web::{App, HttpServer, middleware, web};
use chat_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    let pool = chat_server::db::init("mysql://root@localhost:3306/chat").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .configure(
                chat_server::routes::user::entry,
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 10001))?
        .run()
        .await
}