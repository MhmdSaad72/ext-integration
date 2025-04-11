pub mod shipment_creating;
pub mod store_authorize;
// use serde_json::Value;

use actix_web::web::Data;

use crate::{models::salla_model::SallaWebhook, DbPool};

pub trait EventHandler {
    #[allow(dead_code)]
    fn handle(&self, payload: &SallaWebhook, db_pool: Data<DbPool>) -> Result<(), String>;
}
