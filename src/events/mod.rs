pub mod shipment_creating;
pub mod store_authorize;
use serde_json::Value;

pub trait EventHandler {
    #[allow(dead_code)]
    fn handle(&self, payload: Value) -> Result<(), String>;
}
