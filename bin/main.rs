use actix_web::{App, HttpServer, middleware};
use chat_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    HttpServer::new(|| {
        App::new().configure(
            chat_server::routes::user::entry,
        )
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 10001))?
        .run()
        .await
}