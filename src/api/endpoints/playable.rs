use crate::api::headers::default_headers;
use crate::api::response::handle_response;
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct NexonPlayableRequest<'a> {
    #[serde(rename = "productId")]
    product_id: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NexonPlayableResponse {
    #[serde(rename = "countryCode")]
    country_code: String,
}

pub async fn check_playable(
    client: &Client,
    product_id: &str,
) -> anyhow::Result<NexonPlayableResponse> {
    let endpoint = "https://www.nexon.com/api/game-auth2/v1/playable";
    let headers = default_headers();
    let body = NexonPlayableRequest { product_id };
    let request = client.post(endpoint).headers(headers).json(&body).build()?;
    debug!("\n{:#?}\n{:#?}", request, body);

    let response = client.execute(request).await?;
    handle_response(response).await
}
