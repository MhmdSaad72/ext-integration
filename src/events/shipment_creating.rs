use serde_json::Value;

use super::EventHandler;

pub struct ShipmentCreating;

impl EventHandler for ShipmentCreating {
    fn handle(&self, _payload: Value) -> Result<(), String> {
        println!("Event Handler from shipment creating");
        Ok(())
    }
}
