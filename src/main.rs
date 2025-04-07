use std::{env, io::Result};

use dotenv::dotenv;
use ext_integration::IntegrationApp;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse().expect("PORT must be a number");
    let app = IntegrationApp::new(port);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    app.run(database_url).await
}
