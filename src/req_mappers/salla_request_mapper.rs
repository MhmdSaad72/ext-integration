use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SallaRequestMapper {
    pub order: Order,
    pub shipment: Shipment,
    pub shipping_service: ShippingService,
    pub shipper: Shipper,
    pub consignee: Consignee,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: Option<i64>,
    number: Option<i64>,
    status_slug: String,
    status_label: String,
    currency: String,
    cod_amount: f64,
    payment_method: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Shipment {
    id: i64,
    direction: String,
    number_of_pieces: i8,
    parcels_have_same_dimensions: bool,
    description: String,
    weight: f32,
    weight_unit: String,
    status_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ShippingService {
    carrier: String,
    carrier_service_id: i64,
    shipping_types: String,
    is_international: String,
    is_return: String,
    deliver_to: String,
    servicable_area_id: i8,
}
#[derive(Debug, Serialize, Deserialize)]
struct Shipper {
    name: String,
    company_name: String,
    phone: String,
    email: String,
    address_line: String,
    city: String,
    country: String,
    latitude: f32,
    longitude: f32,
}
#[derive(Debug, Serialize, Deserialize)]
struct Consignee {
    name: String,
    phone: String,
    email: String,
    address_line: String,
    city: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

impl SallaRequestMapper {
    pub fn _new(payload: &Value) -> Result<(), AppError> {
        let _order = Order::try_from(payload)?;
        Ok(())
    }
}

impl Order {
    pub fn try_from(payload: &Value) -> Result<Order, AppError> {
        let order = Order {
            id: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("id")) // Check order.id
                        .or_else(|| v.get("order_id")) // Fall back to order_id
                        .or_else(|| v.get("id")) // Final fallback to id
                })
                .and_then(|v| v.as_i64()),

            number: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("reference_id")) // Check order.number
                        .or_else(|| v.get("order_reference_id")) // Fall back to order_number
                        .or_else(|| v.get("reference_id")) // Final fallback to number
                })
                .and_then(|v| v.as_i64()), // Convert to string,

            status_slug: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("status"))
                        // .and_then(|s| s.get("slug")) // Check order.status_slug
                        .or_else(|| v.get("status"))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),

            status_label: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("status"))
                        .and_then(|s| s.get("name")) // Check order.status_slug
                        .or_else(|| v.get("status").and_then(|o| o.get("name")))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),

            currency: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| {
                            o.get("total").and_then(|s| s.get("currency")).or_else(|| {
                                o.get("amounts")
                                    .and_then(|x| x.get("cash_on_delivery"))
                                    .and_then(|y| y.get("currency"))
                            })
                        })
                        .or_else(|| {
                            v.get("amounts")
                                .and_then(|o| o.get("cash_on_delivery"))
                                .and_then(|y| y.get("currency"))
                        })
                        .or_else(|| v.get("total").and_then(|o| o.get("currency")))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or("SAR".to_string()),

            cod_amount: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("total"))
                        .and_then(|s| s.get("amount"))
                        .or_else(|| v.get("total").and_then(|o| o.get("amount")))
                })
                .or_else(|| payload.get("cod_amount"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),

            payment_method: payload
                .get("data")
                .and_then(|v| {
                    v.get("order")
                        .and_then(|o| o.get("payment_method"))
                        .or_else(|| v.get("payment_method"))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or(AppError::BadRequest {
                    field: "jjjjjjjjjj".to_string(),
                })?,
        };
        Ok(order)
    }
}
