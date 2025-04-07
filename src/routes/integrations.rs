use actix_web::web::{scope, ServiceConfig};

use crate::{
    handlers::salla_handler::handle_webhook_events,
    middlewares::salla_webhook_secret::SallaWebhookAuthorization,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/integrations").service(
            scope("")
                .service(handle_webhook_events)
                .wrap(SallaWebhookAuthorization),
        ),
    );
}
