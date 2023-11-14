use actix_web::{App, HttpServer, middleware, web};
use env_logger::Env;
use sea_orm::DatabaseConnection;
use chat_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = chat_server::db::init("mysql://root@localhost:3306/chat").await.unwrap();
    let state = chat_server::AppState {
        conn: pool
    };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(state.clone()))
            .configure(
                chat_server::routes::user::entry,
            )
    })
        .bind(("127.0.0.1", 10001))?
        .run()
        .await
}