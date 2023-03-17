use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model as aws_model;
use aws_sdk_dynamodb::Client;
use serde;
use serde_json;

pub struct DynamoDBClient<'a> {
    _table: &'a str,
    client: aws_sdk_dynamodb::Client,
}

impl DynamoDBClient<'_> {
    pub async fn new(_table: &str) -> DynamoDBClient {
        let client = get_dynamodb_client().await;

        DynamoDBClient { _table, client }
    }

    pub async fn create_item<T: serde::Serialize>(&self, item: T) -> bool {
        let map = serde_json::to_value(item).unwrap();
        let mut request = self.client.put_item().table_name("table");

        if let serde_json::Value::Object(o) = map {
            for (key, value) in o.iter() {
                request = request.item(key, Self::format_value(value.to_owned()).unwrap());
            }

            let resp = request.send().await.unwrap();
            let attributes = resp.attributes().unwrap();

            let _username = attributes.get("username").cloned();

            return true;
        }

        false
    }

    // fn format<T: serde::Serialize>(&self, item: T) -> () {}

    fn format_value(value: serde_json::Value) -> Option<aws_sdk_dynamodb::model::AttributeValue> {
        match value {
            serde_json::Value::Bool(b) => Some(aws_model::AttributeValue::Bool(b)),
            serde_json::Value::Number(n) => Some(aws_model::AttributeValue::N(n.to_string())),
            serde_json::Value::String(s) => Some(aws_model::AttributeValue::S(s.to_string())),
            _ => None,
        }
    }
}

async fn get_dynamodb_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;

    Client::new(&config)
}
