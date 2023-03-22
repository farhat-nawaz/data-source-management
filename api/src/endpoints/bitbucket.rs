use std::collections::HashMap;

use super::super::{AppState, BitbucketDataSource, DataSource, Response};
use actix_web::{get, web, Responder, Scope};

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

    if let None = BitbucketDataSource::create(conn, "Test".to_owned(), code.to_owned()).await {
        return prepare_response(200, false, "Could not create new source!");
    }

    prepare_response(200, true, "New Bitbucket source created successfully!")
}

fn prepare_response(status_code: i32, success: bool, message: &str) -> Response {
    Response {
        status_code,
        success,
        message,
    }
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("/bitbucket").service(oauth)
}
