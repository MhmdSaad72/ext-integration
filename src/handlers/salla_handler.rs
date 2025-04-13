use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use diesel::{insert_into, prelude::*};
use log::info;
use serde_json::{json, Value};

use crate::{
    errors::app_error::AppError,
    schema::salla_webhooks,
    utilities::{cache::Cache, database::get_db_connection},
    DbPool,
};
use crate::{
    models::salla_model::{NewWebhook, SallaWebhook},
    observers::salla_observer::SallaWebhooksObserver,
};

#[post("salla-plugin/webhooks")]
pub async fn handle_webhook_events(
    conn: Data<DbPool>,
    cache: Data<Cache>,
    body: Json<Value>,
) -> Result<HttpResponse, AppError> {
    info!(target: "salla_plugin", "Salla Plugin Request: {}", json!(body).to_string());
    use self::salla_webhooks::dsl::*;

    let _event = body
        .get("event")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::BadRequest {
            field: "Missing or invalid 'event' field".into(),
        })?;

    let _merchant_id = body
        .get("merchant")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| AppError::BadRequest {
            field: "Missing or invalid 'merchant' field".into(),
        })?;

    let _order_id = body.get("order_id").and_then(|v| v.as_i64());
    let _order_reference_id = body.get("order_id").and_then(|v| v.as_i64());
    let _payload = body.clone();

    let new_hook = NewWebhook {
        event: _event,
        merchant_id: _merchant_id,
        order_id: _order_id,
        order_reference_id: _order_reference_id,
        payload: _payload,
    };
    // Check if the event is already processed
    let conn_clone = conn.clone();
    let result = web::block(move || {
        let connection = &mut get_db_connection(conn)?;

        insert_into(salla_webhooks)
            .values(&new_hook)
            .returning(SallaWebhook::as_returning())
            .get_result(connection)
            .map_err(|e| AppError::from(e))
    })
    .await?;

    // Handle the `Result` properly
    return match result {
        Ok(model) => {
            SallaWebhooksObserver::created(&model, conn_clone, cache).await;
            Ok(HttpResponse::Ok().json(json!({ "success": true, "data": "Received" })))
        }
        Err(e) => Err(e),
    };
}
