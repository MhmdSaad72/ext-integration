use serde_json::Value;

use super::EventHandler;

pub struct StoreAuthorize;
impl EventHandler for StoreAuthorize {
    fn handle(&self, _payload: Value) -> Result<(), String> {
        println!("Event Handler from store authorize");
        Ok(())
    }
}
