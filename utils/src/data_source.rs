use crate::mutation::Mutation;
use crate::query;
use entity::sea_orm::DatabaseConnection;
use std::collections::HashMap;

use async_trait::async_trait;
use uuid;

enum DataSourceType {
    Bitbucket,
    Gitlab,
}

enum AuthenticationType {
    OAuth,
    App,
}

pub struct BitbucketDataSource {
    uuid: String,
    name: String,
    authentication_type: String,
    data_source_type: String,
    access_token: String,
    refresh_token: String,
}

impl OAuth for BitbucketDataSource {
    fn refresh_token(&self) -> String {
        String::from("xxxx-xxx-xxx-xxxx")
    }

    fn get_access_tokens(_oauth_authorization_code: &String) -> Option<(String, String)> {
        Some((
            String::from("xxxx-xxx-xxx-xxxx"),
            String::from("xxxx-xxx-xxx-xxxx"),
        ))
    }
}

#[async_trait]
impl DataSource<BitbucketDataSource> for BitbucketDataSource {
    fn as_persist_hashmap(&self) -> HashMap<String, String> {
        let data = [
            ("uuid".to_owned(), self.uuid.to_owned()),
            ("name".to_owned(), self.name.to_owned()),
            (
                "authentication_type".to_owned(),
                self.authentication_type.to_owned(),
            ),
            (
                "data_source_type".to_owned(),
                self.data_source_type.to_owned(),
            ),
            // ("uuid".to_owned(), self.uuid.to_owned()),
        ];
        HashMap::from(data)
    }
    fn authenticate(&mut self) {
        self.access_token = self.refresh_token();
    }

    async fn create(
        conn: &DatabaseConnection,
        mut properties: HashMap<String, String>,
    ) -> Option<BitbucketDataSource> {
        let uuid = uuid::Uuid::new_v4();

        let (access_token, refrresh_token) =
            Self::get_access_tokens(&properties["oauth_authorization_code"])?;

        properties.insert("uuid".to_string(), uuid.to_string());
        properties.insert("access_token".to_string(), access_token);
        properties.insert("refresh_token".to_string(), refrresh_token);

        let data_source = Self::new(properties)?;

        if let Err(_) = Mutation::create_data_source(conn, data_source.as_persist_hashmap()).await {
            return None;
        }

        Some(data_source)
    }

    fn new(properties: HashMap<String, String>) -> Option<BitbucketDataSource> {
        let uuid = properties.get("uuid")?.clone();
        let name = properties.get("name")?.clone();
        let access_token = properties.get("access_token")?.clone();
        let refresh_token = properties.get("refresh_token")?.clone();
        let authentication_type = properties.get("authentication_type")?.clone();
        let data_source_type = properties.get("data_source_type")?.clone();

        Some(BitbucketDataSource {
            uuid,
            name,
            authentication_type,
            data_source_type,
            access_token,
            refresh_token,
        })
    }

    fn properties(&self) {
        // HashMap
    }
}

#[async_trait]
pub trait DataSource<T> {
    fn as_persist_hashmap(&self) -> HashMap<String, String>;
    fn authenticate(&mut self);
    async fn create(conn: &DatabaseConnection, properties: HashMap<String, String>) -> Option<T>;
    fn new(properties: HashMap<String, String>) -> Option<T>;
    fn properties(&self);
    // fn persist(&self) -> bool;
    // fn toggle_active(&mut self) {
    //     self.is_active = !self.is_active;
    // }
}

trait OAuth {
    fn refresh_token(&self) -> String;
    fn get_access_tokens(_oauth_authorization_code: &String) -> Option<(String, String)>;
}
