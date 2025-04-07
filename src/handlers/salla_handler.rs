use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use diesel::{
    insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use log::info;
use serde_json::{json, Value};

use crate::{errors::app_error::AppError, schema::salla_webhooks, utilities::cache::Cache};
use crate::{
    models::salla_model::{NewWebhook, SallaWebhook},
    observers::salla_observer::SallaWebhooksObserver,
};

#[post("salla-plugin/webhooks")]
pub async fn handle_webhook_events(
    conn: Data<Arc<Pool<ConnectionManager<PgConnection>>>>,
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
    let _result = web::block(move || {
        let connection: &mut diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> =
            &mut conn.get().map_err(|_| AppError::DatabaseError {
                field: "connection".into(),
                source: diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new("Failed to get database connection".to_string()),
                ),
            })?;

        let result: Result<SallaWebhook, AppError> = insert_into(salla_webhooks)
            .values(&new_hook)
            .returning(SallaWebhook::as_returning())
            .get_result(connection)
            .map_err(|e| AppError::from(e));

        let model = result.unwrap();
        SallaWebhooksObserver::created(&model, conn, cache);
        Ok::<_, AppError>(())
    })
    .await?;

    Ok(HttpResponse::Ok().json(json!({ "success": true, "data": "Received" })))
}
