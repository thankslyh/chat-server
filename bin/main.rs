use chat_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    chat_server::start().await
}
