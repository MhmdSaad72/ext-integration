use std::sync::Arc;

use actix_web::web::Data;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use log::error;

use crate::{
    events::{shipment_creating::ShipmentCreating, store_authorize::StoreAuthorize, EventHandler},
    models::salla_model::SallaWebhook,
    schema::salla_webhooks,
    utilities::cache::Cache,
};
pub struct SallaWebhooksObserver;

impl SallaWebhooksObserver {
    #[allow(dead_code)]
    pub fn get_event_handler(event: &str) -> Option<Box<dyn EventHandler>> {
        match event {
            "shipment.creating" => Some(Box::new(ShipmentCreating)),
            "app.store.authorize" => Some(Box::new(StoreAuthorize)),
            _ => None,
        }
    }
    pub fn created(
        model: &SallaWebhook,
        db_pool: Data<Arc<Pool<ConnectionManager<PgConnection>>>>,
        cache: Data<Cache>,
    ) {
        use self::salla_webhooks::dsl::*;
        let cache_key = &Self::generate_cache_key(model);

        if let Some(handler) = Self::get_event_handler(&model.event) {
            if !cache.has(cache_key) {
                cache.insert(cache_key, 60);
                if let Err(_e) = handler.handle(model.payload.clone()) {
                    error!(target: "salla_plugin", "Salla Plugin Webhook Event Named {} Has No Handler", model.event);
                } else {
                    // Update the model as processed
                    let conn = &mut db_pool.get().expect("error");
                    diesel::update(salla_webhooks.find(model.id))
                        .set(processed.eq(true))
                        .execute(conn)
                        .expect("Failed to update payload");
                }
            }
        } else {
            eprintln!("No handler for event: {}", model.event);
        }
    }
    fn generate_cache_key(model: &SallaWebhook) -> String {
        format!("{}-{}", model.event, model.merchant_id)
    }
}
