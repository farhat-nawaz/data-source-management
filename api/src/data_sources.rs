use data_source_macro::async_trait;
pub use data_source_macro::{uuid, DataSource, DataSourceMeta, OAuth};
use data_source_macro_derive::{DataSource, OAuth};
use utils::sea_orm::DatabaseConnection;
use utils::Mutation;

use std::collections::HashMap;

#[derive(OAuth, DataSource)]
pub struct BitbucketDataSource {
    uuid: String,
    name: String,
    authentication_type: String,
    data_source_type: String,
    access_token: String,
    refresh_token: String,
}

#[derive(OAuth, DataSource)]
pub struct GitlabDataSource {
    uuid: String,
    name: String,
    authentication_type: String,
    data_source_type: String,
    access_token: String,
    refresh_token: String,
}

impl DataSourceMeta for BitbucketDataSource {
    fn fields() -> Vec<&'static str> {
        vec!["uuid", "name", "authentication_type", "data_source_type"]
    }
}

impl DataSourceMeta for GitlabDataSource {
    fn fields() -> Vec<&'static str> {
        vec!["uuid", "name", "authentication_type", "data_source_type"]
    }
}
