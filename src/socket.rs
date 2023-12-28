mod server;
mod session;

pub use self::server::ChatServer;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn start(
    req: HttpRequest,
    stream: web::Payload,
    serv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::ChatSession::new(serv.get_ref().clone()),
        &req,
        stream,
    )
}
