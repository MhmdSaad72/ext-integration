use actix_web::web::Data;
use serde_json::json;

use crate::{
    helpers::salla_operations::SallaApiClient,
    models::{integrated_store_model::IntegratedStore, salla_model::SallaWebhook},
    DbPool,
};

use super::EventHandler;

pub struct StoreAuthorize;
impl EventHandler for StoreAuthorize {
    fn handle(&self, model_data: &SallaWebhook, db_pool: Data<DbPool>) -> Result<(), String> {
        let payload = model_data.payload.clone();
        let access_token = payload["access_token"]
            .as_str()
            .ok_or("Access token not found")?;
        let salla_client = SallaApiClient::new(access_token);
        let store_info = salla_client
            .get_store_info()
            .await
            .map_err(|e| format!("Failed to get store info: {}", e))?;

        // println!("Event Handler from store authorize");
        Ok(())
    }
}
