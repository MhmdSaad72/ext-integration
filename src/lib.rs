use std::{io::Result, sync::Arc};

use actix_web::{
    error::JsonPayloadError,
    web::{self, JsonConfig},
    App, HttpRequest, HttpServer,
};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use errors::app_error::AppError;
use loggers::setup::setup_logger;
use routes::integrations::configure;
use utilities::cache::Cache;

mod errors;
mod events;
mod handlers;
mod loggers;
mod middlewares;
mod models;
mod observers;
mod routes;
mod schema;
mod tasks;
mod utilities;

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub struct IntegrationApp {
    pub port: u16,
}

impl IntegrationApp {
    pub fn new(port: u16) -> Self {
        IntegrationApp { port }
    }

    pub async fn run(&self, database_url: String) -> Result<()> {
        let addrs = format!("127.0.0.1:{}", self.port);
        let manager = r2d2::ConnectionManager::<diesel::PgConnection>::new(database_url);
        let db_pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        let db_pool = Arc::new(db_pool);
        setup_logger();
        println!("Starting server at {}", addrs);
        HttpServer::new(move || {
            App::new()
                .app_data(JsonConfig::default().error_handler(handle_payload_error))
                .app_data(web::Data::new(db_pool.clone()))
                .app_data(web::Data::new(Cache::new()))
                .configure(configure)
        })
        .bind(addrs)?
        .run()
        .await
    }
}

pub fn handle_payload_error(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let error_message = AppError::from(err);
    error_message.into()
}
