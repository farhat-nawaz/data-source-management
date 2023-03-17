use actix_web::{get, middleware::Logger, HttpResponse, Responder};

use data_source_management::DynamoDBClient;

#[derive(serde::Serialize)]
struct DataSource {
    uuid: String,
    name: String,
    authentication_type: String,
}

#[tokio::main]
async fn _main() {
    let data_source = DataSource {
        uuid: String::from("xxxx-xxx-xxx-xxxx"),
        name: String::from("Kaira Technologies"),
        authentication_type: String::from("github_app"),
    };

    let ddb_client = DynamoDBClient::new("data_sources").await;

    // create a new data source
    let success = ddb_client.create_item(data_source).await;

    if success {
        println!("Source created successfully, details:")
    } else {
        println!("There was an error.")
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use env_logger;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
