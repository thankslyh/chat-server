use actix::Actor;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_actors::ws;
use chat_server;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://127.0.0.1:10001");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = chat_server::service::init("mysql://root@localhost:3306/chat")
        .await
        .expect("db connect error");
    let user = chat_server::CtxUser {
        id: 1,
        uid: "caf1577c-8029-4594-aa76-3915a9719f6c".to_string(),
    };
    let state = chat_server::AppState {
        conn: pool,
        user: Some(user),
    };

    let serv = chat_server::socket::ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(chat_server::middleware::Authoriser::new(vec![
                "/user/login",
            ]))
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(serv.clone()))
            .configure(chat_server::routes::user::entry)
            .configure(chat_server::routes::friends::entry)
            .route("/ws", web::get().to(chat_server::socket::start))
    })
    .bind(("127.0.0.1", 10001))?
    .run()
    .await
}
