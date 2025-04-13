pub mod shipment_creating;
pub mod store_authorize;
// use serde_json::Value;

use std::error::Error;

use actix_web::web::Data;
use async_trait::async_trait;

use crate::{models::salla_model::SallaWebhook, DbPool};

#[async_trait]
pub trait EventHandler {
    async fn handle(
        &self,
        payload: &SallaWebhook,
        db_pool: Data<DbPool>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
