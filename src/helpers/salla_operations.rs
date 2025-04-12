use actix_web::HttpResponse;
use reqwest::Client;

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

    // pub async fn get_store_info(&self) -> Result<HttpResponse, reqwest::Error> {
    //     let url = format!("{}/store", self.base_url);
    //     let response = self
    //         .client
    //         .get(&url)
    //         .header("Authorization", format!("Bearer {}", self.access_token))
    //         .send()
    //         .await?;

    //     Ok(HttpResponse::Ok().json(response.json::<serde_json::Value>().await?))
    // }
}
