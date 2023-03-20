pub fn add(left: usize, right: usize) -> usize {
    left + right
}
use std::collections::HashMap;

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
    authentication_type: AuthenticationType,
    data_source_type: DataSourceType,
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

impl DataSource<BitbucketDataSource> for BitbucketDataSource {
    fn authenticate(&mut self) {
        self.access_token = self.refresh_token();
    }

    fn create(mut properties: HashMap<String, String>) -> Option<BitbucketDataSource> {
        let uuid = uuid::Uuid::new_v4();

        let (access_token, refrresh_token) =
            Self::get_access_tokens(&properties["oauth_authorization_code"])?;

        properties.insert("uuid".to_string(), uuid.to_string());
        properties.insert("access_token".to_string(), access_token);
        properties.insert("refrresh_token".to_string(), refrresh_token);

        Self::new(properties)
    }

    fn new(properties: HashMap<String, String>) -> Option<BitbucketDataSource> {
        let uuid = properties.get("uuid")?.clone();
        let name = properties.get("name")?.clone();
        let access_token = properties.get("access_token")?.clone();
        let refresh_token = properties.get("refresh_token")?.clone();

        Some(BitbucketDataSource {
            uuid,
            name,
            authentication_type: AuthenticationType::OAuth,
            data_source_type: DataSourceType::Bitbucket,
            access_token,
            refresh_token,
        })
    }

    fn properties(&self) {
        // HashMap
    }
}

trait DataSource<T> {
    fn authenticate(&mut self);
    fn create(properties: HashMap<String, String>) -> Option<T>;
    fn new(properties: HashMap<String, String>) -> Option<T>;
    fn properties(&self);
    // fn toggle_active(&mut self) {
    //     self.is_active = !self.is_active;
    // }
}

trait OAuth {
    fn refresh_token(&self) -> String;
    fn get_access_tokens(_oauth_authorization_code: &String) -> Option<(String, String)>;
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
