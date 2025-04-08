use log::info;
use serde_json::json;

use crate::models::salla_model::SallaWebhook;

use super::EventHandler;

pub struct StoreAuthorize;
impl EventHandler for StoreAuthorize {
    fn handle(&self, model_data: &SallaWebhook) -> Result<(), String> {
        let merchant_id = model_data.merchant_id;
        let mut payload = model_data.payload.clone();
        payload["platform"] = json!("salla_plugin");
        payload["shop_id"] = json!(merchant_id);
        // info!(target: "salla_plugin", "Store Authorize payload: {}", json!(payload).to_string());
        // println!("Event Handler from store authorize");
        Ok(())
    }
}
