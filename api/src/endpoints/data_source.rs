use actix_web::{get, web, HttpResponse, Responder};

#[get("/all")]
async fn all() -> impl Responder {
    HttpResponse::Ok().body("All Data Sources!")
}

#[get("/authentication-tokens")]
async fn authentication_tokens() -> impl Responder {
    HttpResponse::Ok().body("All Authentication Tokens!")
}

#[get("/{uuid}")]
async fn one_by_uuid(info: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Data Source uuid: {}", info.into_inner().0))
}

#[get("/customer/{uuid}")]
async fn all_by_customer_uuid(info: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Data Sources with Customer uuid: {}",
        info.into_inner().0
    ))
}

pub fn get_scoped_handlers() -> actix_web::Scope {
    web::scope("/data-sources")
        .service(all)
        .service(one_by_uuid)
        .service(all_by_customer_uuid)
        .service(authentication_tokens)
}

// /api/data-sources/all
// /api/data-sources/authentication-tokens
// /api/data-sources/uuid
// /api/data-sources/customer/uuid
