use std::collections::HashMap;

use super::super::AppState;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use utils::{BitbucketDataSource, DataSource};

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

    let properties = HashMap::from([
        ("name".to_owned(), "Test".to_owned()),
        ("authentication_type".to_owned(), "app".to_owned()),
        ("data_source_type".to_owned(), "bitbucket".to_owned()),
    ]);

    if let None = BitbucketDataSource::create(conn, properties).await {
        return HttpResponse::Ok().body("Could not create new source!");
    }

    HttpResponse::Ok().body("New Bitbucket source created successfully!")
}

pub fn get_scoped_handlers() -> Scope {
    web::scope("/bitbucket").service(oauth)
}
