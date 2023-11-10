use actix_web::Responder;
use super::*;
use sqlx::MySqlPool;
use crate::Context;

const PREFIX: &'static str = "/conversation";

pub struct CreateForm {

}
#[post("/create")]
async fn create(pool: web::Data<MySqlPool>, context: web::Data<Context>) -> actix_web::Result<impl Responder> {

}