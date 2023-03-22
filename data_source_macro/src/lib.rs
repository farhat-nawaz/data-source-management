use std::collections::HashMap;

pub use async_trait::async_trait;
pub use uuid;

use entity::sea_orm::DatabaseConnection;

enum _DataSourceType {
    Bitbucket,
    Gitlab,
}

enum _AuthenticationType {
    OAuth,
    App,
}

#[async_trait]
pub trait DataSource<T> {
    fn as_persist_hashmap(&self) -> HashMap<&str, String>;
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

pub trait OAuth {
    fn refresh_token(&self) -> String;
    fn get_access_tokens(_oauth_authorization_code: &String) -> Option<(String, String)>;
}
