use diesel::prelude::*;
use serde_json::Value;

use crate::schema::salla_webhooks;

#[derive(Insertable)]
#[diesel(table_name = salla_webhooks)]
pub struct NewWebhook {
    pub event: String,
    pub merchant_id: i64,
    pub order_id: Option<i64>,
    pub order_reference_id: Option<i64>,
    pub payload: Value,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = salla_webhooks)]
#[allow(dead_code)]
pub struct SallaWebhook {
    pub id: i64,
    pub event: String,
    pub merchant_id: i64,
    pub order_id: Option<i64>,
    pub order_reference_id: Option<i64>,
    pub payload: Value,
}
