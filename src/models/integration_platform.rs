use crate::{
    errors::app_error::AppError, schema::integration_platforms,
    utilities::database::get_db_connection, DbPool,
};
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde_json::Value;

#[derive(DbEnum, Debug, Clone, Copy, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AvailabilityStatus"]
pub enum AvailabilityStatus {
    Available,
    NotAvailable,
    Soon,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = integration_platforms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct IntegrationPlatform {
    pub id: i64,
    pub platform: String,
    pub label: Option<String>,
    pub gateway: Option<String>,
    pub required_fields: Option<Value>,
    pub gateway_requirements: Option<String>,
    pub is_ready: bool,
    pub enabled: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub integration_abilities: Value,
    pub def_store_driver: Option<String>,
    pub reqs_mapper_class: Option<String>,
    pub def_order_proc_method: String,
    pub order_proc_changeable: bool,
    pub img: Option<String>,
    pub public_showable: bool,
    pub video_guide_url: Option<String>,
    pub group_id: Option<i64>,
    pub img_light: Option<String>,
    pub img_max_height: Option<String>,
    pub availability_status: AvailabilityStatus,
    pub guide_docs: Option<Value>,
}

impl IntegrationPlatform {
    pub async fn find_by_name(name: String, db_pool: &Data<DbPool>) -> Result<Self, AppError> {
        use self::integration_platforms::dsl::*;
        let connection = &mut get_db_connection(db_pool)?;

        let integration_platform = integration_platforms
            .filter(platform.eq(name))
            .select(IntegrationPlatform::as_select())
            .first::<IntegrationPlatform>(connection)?;
        Ok(integration_platform)
    }
}
