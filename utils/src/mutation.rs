use std::collections::HashMap;

use ::entity::data_source;
use ::entity::sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_data_source(
        db: &DbConn,
        data: HashMap<&str, String>,
    ) -> Result<data_source::Model, DbErr> {
        // let data_source: data_source::ActiveModel = data.iter()
        // .map(|(k, v)| (k.as_str(), v.as_str()))
        // .collect::<Vec<_>>()
        // .try_into()
        // .unwrap();

        data_source::ActiveModel {
            uuid: Set(data["uuid"].to_owned()),
            name: Set(data["name"].to_owned()),
            authentication_type: Set(data["authentication_type"].to_owned()),
            data_source_type: Set(data["data_source_type"].to_owned()),
            // properties: data["properties"],
        }
        .insert(db)
        .await
    }
}
