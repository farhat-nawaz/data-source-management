use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn get_scoped_handlers() -> actix_web::Scope {
    web::scope("").service(hello)
}
