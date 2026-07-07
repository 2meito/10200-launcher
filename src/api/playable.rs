use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct PlayableRequest<'a> {
    #[serde(rename = "productId")]
    product_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PlayableResponse {
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

pub async fn check_playable(
    client: &reqwest::Client,
    product_id: i32,
) -> Result<PlayableResponse, Box<dyn Error>> {
    let endpoint = "https://www.nexon.com/api/game-auth2/v1/playable";
    let body = PlayableRequest { product_id: &product_id.to_string() };
    let response = client.post(endpoint).json(&body).send().await?;
    Ok(response.json().await?)
}