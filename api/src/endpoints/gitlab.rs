use std::collections::HashMap;

use super::super::{AppState, Response};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use utils::{DataSource, GitlabDataSource};

#[get("/oauth")]
async fn oauth(
    query_params: web::Query<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    if !query_params.contains_key("code") {
        return HttpResponse::BadRequest().body("`code` parameter is required");
    }

    let code = query_params["code"].to_owned();
    let conn = &data.conn;
    let mut success = true;
    let mut message = "New Gitlab source created successfully!";

    if let None = GitlabDataSource::create(conn, "Test".to_owned(), code.to_owned()).await {
        success = false;
        message = "Could not create new source!";
    }

    HttpResponse::Ok().json(Response { success, message })
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("/gitlab").service(oauth)
}
