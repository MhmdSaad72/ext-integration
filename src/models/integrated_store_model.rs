use crate::schema::integrated_stores;
use actix_web::web::Data;
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

use crate::{errors::app_error::AppError, DbPool};
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = integrated_stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IntegratedStore {
    pub id: i64,
    pub user_id: i64,
    pub integration_platform_id: i64,
    pub is_stopped: bool,
    pub is_disabled: bool,
    pub store_url: String,
    pub store_name: String,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub shop_id: Option<String>,
    pub default_carrier_id: Option<i64>,
    pub default_shipping_address_id: Option<i64>,
    pub integration_info: Option<String>,
    pub integration_token_data: Option<String>,
    pub odd_enabled: bool,
    pub whpm_enabled: bool,
    pub webhook_authorization: String,
    pub account_id: Option<i64>,
    pub reqs_mapper_class: Option<String>,
    pub integration_abilities: Value,
    pub order_proc_method: Option<String>,
    pub group_id: Option<i64>,
    pub auth_code: Option<String>,
    pub authorization_code: Option<String>,
}

impl IntegratedStore {
    pub fn find_by_shop_id(_shop_id: String, conn: Data<DbPool>) -> Result<Self, AppError> {
        use self::integrated_stores::dsl::*;
        let connection = &mut conn.get().map_err(|_| AppError::DatabaseError {
            field: "connection".into(),
            source: diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Failed to get database connection".to_string()),
            ),
        })?;

        let integrated_store = integrated_stores
            .filter(shop_id.eq(_shop_id))
            .select(IntegratedStore::as_select())
            .first::<IntegratedStore>(connection)
            .map_err(|_| AppError::DatabaseError {
                field: "shop_id".into(),
                source: diesel::result::Error::NotFound,
            })?;
        Ok::<IntegratedStore, AppError>(integrated_store)
    }
}
