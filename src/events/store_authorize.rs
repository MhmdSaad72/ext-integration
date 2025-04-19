use std::error::Error;

use actix_web::web::Data;
use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;

use crate::{
    helpers::salla_operations::SallaApiClient,
    models::{
        integrated_store_model::IntegratedStore, salla_model::SallaWebhook,
        temp_store_integration::TempStoreIntegration,
    },
    DbPool,
};

use super::EventHandler;

pub struct StoreAuthorize;

#[async_trait]
impl EventHandler for StoreAuthorize {
    async fn handle(
        &self,
        model_data: &SallaWebhook,
        db_pool: Data<DbPool>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let payload = model_data.payload.clone();
        let access_token = payload["data"]["access_token"]
            .as_str()
            .ok_or("Access token not found")?;
        let salla_client = SallaApiClient::new(access_token);

        let salla_response = salla_client.get_store_info().await;
        let store_info = salla_response.map_err(|e| {
            error!(target: "salla_plugin", "Failed to get store info: {}", e);
            "Failed to get store info"
        })?;

        let data = store_info.json::<Value>().await?;
        let shop_id = data
            .get("data")
            .and_then(|d| d.get("id"))
            .and_then(|id| id.as_i64())
            .ok_or("Shop ID not found")?;

        let store_exit = IntegratedStore::find_by_shop_id(shop_id.to_string(), &db_pool).await;
        if store_exit.is_ok() {
            info!(target: "salla_plugin", "Store already exists, updating...");
            store_exit.unwrap().update_store(data, &db_pool).await?;
            return Ok(());
        }
        info!(target: "salla_plugin", "Store not found, creating new store...");
        TempStoreIntegration::update_or_create(data, payload, &db_pool).await?;
        Ok(())
    }
}
