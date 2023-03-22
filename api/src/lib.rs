use std::env;

use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use utils::sea_orm::{Database, DatabaseConnection};
mod endpoints;

use data_source_macro::async_trait;
use data_source_macro::{uuid, DataSource, OAuth};
use data_source_macro_derive::{DataSource, OAuth};
use utils::Mutation;

use std::collections::HashMap;

#[derive(OAuth, DataSource)]
pub struct BitbucketDataSource {
    uuid: String,
    name: String,
    authentication_type: String,
    data_source_type: String,
    access_token: String,
    refresh_token: String,
}

#[derive(OAuth, DataSource)]
pub struct GitlabDataSource {
    uuid: String,
    name: String,
    authentication_type: String,
    data_source_type: String,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[derive(Serialize)]
pub struct Response<'a> {
    status_code: i32,
    success: bool,
    message: &'a str,
}

impl Responder for Response<'_> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.status_code {
            200 => HttpResponse::Ok().json(self),
            400 => HttpResponse::BadRequest().json(self),
            _ => HttpResponse::Unauthorized().json(self),
        }
    }
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    use actix_web::middleware::Logger;
    use actix_web::web;
    use actix_web::{App, HttpServer};
    use dotenv;
    use env_logger;

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(endpoints::index::get_scoped_handlers())
                    .service(endpoints::data_source::get_scoped_handlers()),
            )
            .service(endpoints::bitbucket::get_scoped_handlers())
            .service(web::scope("/gitlab").service(endpoints::gitlab::get_scoped_handlers()))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
        // .wrap(Logger::new("%a %T"))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
