// use data_source_management::DynamoDBClient;

use api;

#[derive(serde::Serialize)]
struct DataSource {
    uuid: String,
    name: String,
    authentication_type: String,
}

// #[tokio::main]
// async fn _main() {
//     let data_source = DataSource {
//         uuid: String::from("xxxx-xxx-xxx-xxxx"),
//         name: String::from("Kaira Technologies"),
//         authentication_type: String::from("github_app"),
//     };

//     let ddb_client = DynamoDBClient::new("data_sources").await;

//     // create a new data source
//     let success = ddb_client.create_item(data_source).await;

//     if success {
//         println!("Source created successfully, details:")
//     } else {
//         println!("There was an error.")
//     }
// }

fn main() -> std::io::Result<()> {
    api::start()
}
