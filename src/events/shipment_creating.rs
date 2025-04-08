// use serde_json::Value;

use crate::models::salla_model::SallaWebhook;

use super::EventHandler;

pub struct ShipmentCreating;

impl EventHandler for ShipmentCreating {
    fn handle(&self, _payload: &SallaWebhook) -> Result<(), String> {
        println!("Event Handler from shipment creating");
        Ok(())
    }
}
