use log::error;
use reqwest::{Client, Response};
use std::error::Error;

pub struct SallaApiClient {
    client: Client,
    base_url: String,
    access_token: String,
}

impl SallaApiClient {
    pub fn new(access_token: &str) -> Self {
        let client = Client::new();
        SallaApiClient {
            client,
            base_url: "https://api.salla.dev/admin/v2/".to_string(),
            access_token: access_token.to_string(),
        }
    }

    pub async fn get_store_info(&self) -> Result<Response, Box<dyn Error + Send + Sync>> {
        let url = format!("{}store/info", self.base_url);
        self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| {
                error!(target: "salla_plugin", "Failed to get store info: {:?}", e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })
    }
}
