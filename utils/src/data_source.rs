use crate::mutation::Mutation;
use crate::query;
use entity::sea_orm::DatabaseConnection;
use std::collections::HashMap;

use async_trait::async_trait;
use uuid;

enum _DataSourceType {
    Bitbucket,
    Gitlab,
}

enum _AuthenticationType {
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

pub struct GitlabDataSource {
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

impl OAuth for GitlabDataSource {
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
        name: String,
        oauth_authorization_code: String,
    ) -> Option<BitbucketDataSource> {
        let uuid = uuid::Uuid::new_v4();

        let (access_token, refresh_token) = Self::get_access_tokens(&oauth_authorization_code)?;

        let data_source = Self::new(
            uuid.to_string(),
            name,
            String::from("oauth"),
            String::from("gitlab"),
            access_token,
            refresh_token,
        )?;

        if let Err(_) = Mutation::create_data_source(conn, data_source.as_persist_hashmap()).await {
            return None;
        }

        Some(data_source)
    }

    fn new(
        uuid: String,
        name: String,
        authentication_type: String,
        data_source_type: String,
        access_token: String,
        refresh_token: String,
    ) -> Option<BitbucketDataSource> {
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
impl DataSource<GitlabDataSource> for GitlabDataSource {
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
        ];
        HashMap::from(data)
    }
    fn authenticate(&mut self) {
        self.access_token = self.refresh_token();
    }

    async fn create(
        conn: &DatabaseConnection,
        name: String,
        oauth_authorization_code: String,
    ) -> Option<GitlabDataSource> {
        let uuid = uuid::Uuid::new_v4();

        let (access_token, refresh_token) = Self::get_access_tokens(&oauth_authorization_code)?;

        let data_source = Self::new(
            uuid.to_string(),
            name,
            String::from("oauth"),
            String::from("gitlab"),
            access_token,
            refresh_token,
        )?;

        if let Err(_) = Mutation::create_data_source(conn, data_source.as_persist_hashmap()).await {
            return None;
        }

        Some(data_source)
    }

    fn new(
        uuid: String,
        name: String,
        authentication_type: String,
        data_source_type: String,
        access_token: String,
        refresh_token: String,
    ) -> Option<GitlabDataSource> {
        Some(GitlabDataSource {
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

    async fn create(
        conn: &DatabaseConnection,
        name: String,
        oauth_authorization_code: String,
    ) -> Option<T>;

    fn new(
        uuid: String,
        name: String,
        authentication_type: String,
        data_source_type: String,
        access_token: String,
        refresh_token: String,
    ) -> Option<T>;

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
