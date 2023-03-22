use std::env;

use serde::Serialize;
use utils::sea_orm::{Database, DatabaseConnection};
mod endpoints;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[derive(Serialize)]
pub struct Response<'a> {
    success: bool,
    message: &'a str,
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
