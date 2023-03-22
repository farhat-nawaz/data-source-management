use std::collections::HashMap;

use super::super::{AppState, Response};
use actix_web::{get, web, Responder, Scope};
use utils::{DataSource, GitlabDataSource};

#[get("/oauth")]
async fn oauth(
    query_params: web::Query<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    if !query_params.contains_key("code") {
        return prepare_response(400, false, "`code` parameter is required");
    }

    let code = query_params["code"].to_owned();
    let conn = &data.conn;

    if let None = GitlabDataSource::create(conn, "Test".to_owned(), code.to_owned()).await {
        return prepare_response(200, false, "Could not create new source!");
    }

    prepare_response(200, true, "New Gitlab source created successfully!")
}

fn prepare_response(status_code: i32, success: bool, message: &str) -> Response {
    Response {
        status_code,
        success,
        message,
    }
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("/gitlab").service(oauth)
}
