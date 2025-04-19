use std::fmt::Debug;

use actix_web::{
    error::{BlockingError, JsonPayloadError},
    HttpResponse, ResponseError,
};
use derive_more::{derive::Display, Error};
use diesel::result::Error as DieselError;
use serde_json::json;

#[derive(Debug, Display, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[display("Bad Request: {field}")]
    BadRequest { field: String },
    #[display("Invalid Payload")]
    JsonPayloadError,
    #[display("Database error: {field}")]
    DatabaseError { field: String, source: DieselError },
    #[display("An internal error occurred while processing your request. Please try again later.")]
    BlockingError,
    #[display("Invalid data provided")]
    InvalidData,
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        AppError::DatabaseError {
            field: error.to_string(),
            source: error,
        }
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(_value: JsonPayloadError) -> Self {
        AppError::JsonPayloadError
    }
}

impl From<BlockingError> for AppError {
    fn from(_value: BlockingError) -> Self {
        AppError::BlockingError
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::JsonPayloadError => HttpResponse::BadRequest(),
            AppError::DatabaseError { .. } => HttpResponse::InternalServerError(),
            AppError::BlockingError => HttpResponse::InternalServerError(),
            AppError::BadRequest { .. } => HttpResponse::BadRequest(),
            AppError::InvalidData => HttpResponse::BadRequest(),
        };
        builder.json(json!({"sucees":false, "error":err}))
    }
}
