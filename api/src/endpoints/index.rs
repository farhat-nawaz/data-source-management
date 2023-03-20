use actix_web::{get, web, HttpResponse, Responder, Scope};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("").service(hello)
}
