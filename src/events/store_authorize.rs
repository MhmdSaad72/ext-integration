use std::error::Error;

use actix_web::web::Data;
use async_trait::async_trait;
use log::error;
use serde_json::Value;

use crate::{
    helpers::salla_operations::SallaApiClient,
    models::{integrated_store_model::IntegratedStore, salla_model::SallaWebhook},
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

        let shop_id = data["data"]["id"].as_str().ok_or("Shop ID not found")?;
        let store_exit = IntegratedStore::find_by_shop_id(shop_id.to_string(), db_pool).await;
        if store_exit.is_ok() {
            error!(target: "salla_plugin", "Store already exists");
            return Err("Store already exists".into());
        }
        // let shop_id =
        // error!(target: "salla_plugin", "Store info: {:?}", store_info.json::<Value>().await);
        // if store_info. {
        //     error!(target: "salla_plugin", "Failed to get store info: {}", store_info.unwrap_err());
        //     return Err("Failed to get store info".into());
        // }

        Ok(())
    }
}
