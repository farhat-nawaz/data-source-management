use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/oauth")]
async fn oauth(query_params: web::Query<HashMap<String, String>>) -> impl Responder {
    if !query_params.contains_key("code") {
        return HttpResponse::BadRequest().body("`code` parameter is required");
    }

    let _code = query_params["code"].to_owned();

    HttpResponse::Ok().body("New Bitbucket source created successfully!")
}

pub fn get_scoped_handlers() -> actix_web::Scope {
    web::scope("/bitbucket").service(oauth)
}
