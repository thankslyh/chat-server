use super::*;

const PREFIX: &'static str = "/user";
#[get("/list")]
async fn user_list() -> HttpResponse {
    HttpResponse::Ok().body("test")
}

#[get("/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", path);
    HttpResponse::Ok().body("test")
}

#[post("/create")]
async fn user_create(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

#[post("/update/{id}")]
async fn user_update(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

#[post("/delete/{id}")]
async fn user_delete(info: web::Path<(u32,)>) -> HttpResponse {
    println!("path:{:?}", info);
    HttpResponse::Ok().body("test")
}

pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(PREFIX)
            .service(user_list)
            .service(user_detail)
            .service(user_create)
            .service(user_update)
            .service(user_delete)
    );
}