// use serde_json::Value;

use std::error::Error;

use actix_web::web::Data;
use async_trait::async_trait;
use serde_json::json;

use crate::{
    models::{integrated_store_model::IntegratedStore, salla_model::SallaWebhook},
    DbPool,
};

use super::EventHandler;

pub struct ShipmentCreating;

#[async_trait]
impl EventHandler for ShipmentCreating {
    async fn handle(
        &self,
        model_data: &SallaWebhook,
        db_pool: Data<DbPool>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let merchant_id = model_data.merchant_id;
        let mut payload = model_data.payload.clone();
        payload["platform"] = json!("salla_plugin");
        payload["shop_id"] = json!(merchant_id);
        let store = IntegratedStore::find_by_shop_id(merchant_id.to_string(), &db_pool).await;

        if store.is_err() {
            return Err(format!("Order ({}) received and will be ignored, Store Ability to create awbs from API requests is disabled", payload["order_id"]).into());
        }

        Ok(())
    }
}
