use actix_web::web::Data;
use diesel::prelude::*;
use log::error;

use crate::{
    events::{shipment_creating::ShipmentCreating, store_authorize::StoreAuthorize, EventHandler},
    models::salla_model::SallaWebhook,
    schema::salla_webhooks,
    utilities::cache::Cache,
    DbPool,
};
pub struct SallaWebhooksObserver;
type EventHandlerBox = Box<dyn EventHandler + Send + Sync>;

impl SallaWebhooksObserver {
    pub fn get_event_handler(event: &str) -> Option<EventHandlerBox> {
        match event {
            "shipment.creating" => Some(Box::new(ShipmentCreating)),
            "app.store.authorize" => Some(Box::new(StoreAuthorize)),
            _ => None,
        }
    }
    pub async fn created(model: &SallaWebhook, db_pool: Data<DbPool>, cache: Data<Cache>) {
        use self::salla_webhooks::dsl::*;
        let cache_key = &Self::generate_cache_key(model);

        if let Some(handler) = Self::get_event_handler(&model.event) {
            if !cache.has(cache_key) {
                cache.insert(cache_key, 60);

                if let Err(e) = handler.handle(model, db_pool.clone()).await {
                    error!(target: "salla_plugin", "Failed to handle event: {}, {}", model.event, e);
                } else {
                    let conn = &mut db_pool.get().expect("error");
                    diesel::update(salla_webhooks.find(model.id))
                        .set(processed.eq(true))
                        .execute(conn)
                        .map_err(|_| {
                            error!(target: "salla_plugin", "Failed to update webhook status");
                        })
                        .unwrap();
                }
            }
        } else {
            error!(target: "salla_plugin", "No handler for event: {}", model.event);
        }
    }
    fn generate_cache_key(model: &SallaWebhook) -> String {
        format!("{}-{}", model.event, model.merchant_id)
    }
}
