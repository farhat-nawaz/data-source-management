pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use actix_web::middleware::Logger;
use actix_web::web;

mod endpoints;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use env_logger;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(endpoints::index::get_scoped_handlers())
                    .service(endpoints::data_source::get_scoped_handlers()),
            )
            .service(web::scope("/bitbucket").service(endpoints::bitbucket::get_scoped_handlers()))
            .service(web::scope("/gitlab").service(endpoints::gitlab::get_scoped_handlers()))
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
