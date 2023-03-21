use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder, Scope};
use utils::sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/oauth")]
async fn oauth(
    query_params: web::Query<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    if !query_params.contains_key("code") {
        return HttpResponse::BadRequest().body("`code` parameter is required");
    }

    let _code = query_params["code"].to_owned();
    let conn = &data.conn;

    HttpResponse::Ok().body("New Bitbucket source created successfully!")
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("/gitlab").service(oauth)
}
