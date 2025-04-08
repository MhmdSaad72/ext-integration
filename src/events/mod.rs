pub mod shipment_creating;
pub mod store_authorize;
// use serde_json::Value;

use crate::models::salla_model::SallaWebhook;

pub trait EventHandler {
    #[allow(dead_code)]
    fn handle(&self, payload: &SallaWebhook) -> Result<(), String>;
}
