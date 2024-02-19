#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use chat_server;
    #[actix_web::test]
    async fn send_verify_code() {
        let app = test::init_service(chat_server::start().await)
    }
}
