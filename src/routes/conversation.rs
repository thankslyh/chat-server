use actix_web::Responder;
use super::*;
use sqlx::MySqlPool;

const PREFIX: &'static str = "/conversation";

pub struct CreateForm {

}
#[post("/create")]
async fn create(pool: web::Data<MySqlPool>) -> actix_web::Result<impl Responder> {
    Ok("123")
}