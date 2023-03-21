pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::env;

use actix_web::middleware::Logger;
use actix_web::web;

use utils::sea_orm::{Database, DatabaseConnection};

mod endpoints;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use env_logger;

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let conn = Database::connect(&db_url).await.unwrap();

    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(endpoints::index::get_scoped_handlers())
                    .service(endpoints::data_source::get_scoped_handlers()),
            )
            .service(web::scope("/bitbucket").service(endpoints::bitbucket::get_scoped_handlers()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
