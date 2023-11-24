use actix_web::{middleware, web, App, HttpServer};
use chat_server;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = chat_server::service::init("mysql://root@localhost:3306/chat")
        .await
        .unwrap();
    let user = chat_server::CtxUser {
        id: 1,
        uid: "caf1577c-8029-4594-aa76-3915a9719f6c".to_string()
    };
    let state = chat_server::AppState {
        conn: pool,
        user: Some(user),
    };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(state.clone()))
            .configure(chat_server::routes::user::entry)
    })
    .bind(("127.0.0.1", 10001))?
    .run()
    .await
}
