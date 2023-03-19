use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct OAuthQueryString {
    code: String,
}

#[get("/oauth/bitbucket")]
async fn oauth(query_string: web::Query<OAuthQueryString>) -> impl Responder {
    let _code = query_string.code.to_owned();
    HttpResponse::Ok().body("Hello world!")
}

#[get("/oauth/bitbucket2")]
async fn oauth2(query_params: web::Query<HashMap<String, String>>) -> impl Responder {
    let _code = query_params["code"].to_owned();
    HttpResponse::Ok().body("Hello world!")
}
