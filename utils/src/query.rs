use ::entity::sea_orm::*;
use ::entity::{data_source, data_source::Entity as DataSource};

pub struct Query;

impl Query {
    pub async fn find_data_source_by_uuid(
        db: &DbConn,
        uuid: String,
    ) -> Result<Option<data_source::Model>, DbErr> {
        DataSource::find_by_id(uuid).one(db).await
    }
}
